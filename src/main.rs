#![deny(warnings)]

mod stat;

use std::env;
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use std::time::Duration;
use sysinfo::{Pid, RefreshKind, System};

fn print_help() {
    // 读取命令行参数
    let args: Vec<String> = env::args().collect();

    eprintln!("Usage: {} <PID> [Options]", args[0]);
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

    // 退出进程
    process::exit(1);
}

fn print_version() {
    let version = format!("v{}", env!("CARGO_PKG_VERSION"));
    eprintln!("{}", version);
    process::exit(0);
}

fn exit_when_become_orphan_processes() {
    eprintln!("Parent process exited. Exiting...");
    process::exit(0);
}

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running); // 克隆Arc引用
    let (tx, rx) = mpsc::channel::<()>();

    ctrlc::set_handler(move || {
        running_clone.store(false, Ordering::SeqCst);
        let _ = tx.send(()); // 发送信号，忽略发送错误（接收方可能已经退出）
    })
    .expect("Error setting Ctrl-C handler");

    // 读取命令行参数
    let args: Vec<String> = env::args().collect();
    let mut interval = 5; // 默认间隔时间为5秒

    // 查找--interval参数并解析其值
    for (i, arg) in args.iter().enumerate() {
        if arg == "--version" || arg == "-V" {
            print_version();
        } else if arg == "--help" || arg == "--h" {
            print_help();
            return;
        } else if arg == "--interval" && i + 1 < args.len() {
            interval = args[i + 1].parse::<u64>().unwrap_or(5); // 如果解析失败，使用默认值5秒
        }
    }

    if args.len() < 2 {
        eprintln!("Missing PID argument");
        print_help();
        return;
    }

    // 解析 PID
    let target_pid = match args[1].parse() {
        Ok(pid) => pid,
        Err(_) => {
            eprintln!("Invalid PID: {}", args[1]);
            return;
        }
    };

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

        if let Some(root) = stat::build_process_tree(&system, Pid::from_u32(target_pid)) {
            // print_process_tree(&root, 0);
            // 使用 serde_json 序列化 ProcessNode 为 JSON
            let json = serde_json::to_string(&root).unwrap();
            println!("{}", json);
        } else {
            eprintln!("No process found with PID: {}", target_pid);
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
