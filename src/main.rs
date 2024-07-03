mod stat;

use std::env;
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use std::time::Duration;
use sysinfo::{Pid, RefreshKind, System};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_help_and_exit(&args[0]);
    }

    let mut pid = 0;
    let mut interval = 5; // 默认更新间隔为5秒

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--help" | "-h" => print_help_and_exit(&args[0]),
            "--version" | "-V" => print_version_and_exit(),
            "--interval" => {
                if i + 1 < args.len() {
                    interval = args[i + 1].parse::<u64>().unwrap_or_else(|_| {
                        eprintln!("Invalid interval: {}", args[i + 1]);
                        process::exit(1);
                    });
                    i += 1;
                } else {
                    eprintln!("--interval expects a value");
                    process::exit(1);
                }
            }
            _ => {
                if pid == 0 {
                    pid = args[i].parse::<u32>().unwrap_or_else(|_| {
                        eprintln!("Invalid PID: {}", args[i]);
                        process::exit(1);
                    });
                } else {
                    eprintln!("Unexpected argument: {}", args[i]);
                    process::exit(1);
                }
            }
        }
        i += 1;
    }

    if pid == 0 {
        eprintln!("PID is required.");
        print_help_and_exit(&args[0]);
    }

    // 这里是监控逻辑
    monitor_process(pid, interval);
}

fn print_help_and_exit(program_name: &str) {
    eprintln!("Usage: {} <PID> [Options]", program_name);
    eprintln!("Options:");
    eprintln!("  <PID>                Process ID to monitor.");
    eprintln!("  --help,-h            Print the help information");
    eprintln!("  --version,-V         Print the version information");
    eprintln!("  --interval SECONDS   Time interval between updates in seconds.");
    eprintln!("Description:");
    eprintln!("  This tool monitors the specified process and refreshes the information");
    eprintln!("  every SECONDS seconds, as specified by the --interval option.");
    eprintln!("Source Code:");
    eprintln!("  https://github.com/axetroy/ps-tree.rs");
    process::exit(1);
}

fn print_version_and_exit() {
    eprintln!("v{}", env!("CARGO_PKG_VERSION"));
    process::exit(0);
}

fn exit_when_become_orphan_processes() {
    eprintln!("Parent process exited. Exiting...");
    process::exit(0);
}

fn monitor_process(pid: u32, interval: u64) {
    let running = Arc::new(AtomicBool::new(true));
    let _running_clone = Arc::clone(&running);

    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running); // 克隆Arc引用
    let (tx, rx) = mpsc::channel::<()>();

    ctrlc::set_handler(move || {
        running_clone.store(false, Ordering::SeqCst);
        let _ = tx.send(()); // 发送信号，忽略发送错误（接收方可能已经退出）
    })
    .expect("Error setting Ctrl-C handler");

    let current_pid = process::id(); // 获取当前进程的PID，即父进程PID

    let mut system = System::new_with_specifics(RefreshKind::new());

    while running.load(Ordering::SeqCst) {
        system.refresh_all();

        // 检查父进程是否还活着
        match system.process(Pid::from_u32(current_pid)) {
            Some(ps) => match ps.parent() {
                Some(parent) => {
                    if parent.as_u32() == 1 {
                        exit_when_become_orphan_processes()
                    }
                }
                None => exit_when_become_orphan_processes(),
            },
            None => exit_when_become_orphan_processes(),
        }

        if let Some(root) = stat::build_process_tree(&system, Pid::from_u32(pid)) {
            // print_process_tree(&root, 0);
            // 使用 serde_json 序列化 ProcessNode 为 JSON
            let json = serde_json::to_string(&root).unwrap();
            println!("{}", json);
        } else {
            eprintln!("No process found with PID: {}", pid);
        }

        // 使用 recv_timeout 代替 thread::sleep
        match rx.recv_timeout(Duration::from_secs(interval)) {
            Ok(_) | Err(mpsc::RecvTimeoutError::Disconnected) => {
                eprintln!("Exiting due to CTRL+C or channel disconnect...");
                break;
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                // 超时，继续执行
            }
        }
    }
}
