#![deny(warnings)]

mod stat;

use std::env;
use std::process;
use std::thread;
use std::time::Duration;
use std::{
    sync::atomic::{AtomicBool, Ordering},
    sync::Arc,
};
use sysinfo::{Pid, System};

fn print_help() {
    // 读取命令行参数
    let args: Vec<String> = env::args().collect();

    eprintln!("Usage: {} <PID> [Options]", args[0]);
    eprintln!("Options:");
    eprintln!("  <PID>                Process ID to monitor.");
    eprintln!("  --interval SECONDS   Time interval between updates in seconds.\n");
    eprintln!("Description:");
    eprintln!("  This tool monitors the specified process and refreshes the information");
    eprintln!("  every SECONDS seconds, as specified by the --interval option.");

    // 退出进程
    process::exit(1);
}

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");
    // 读取命令行参数
    let args: Vec<String> = env::args().collect();
    let mut interval = 5; // 默认间隔时间为5秒

    // 查找--interval参数并解析其值
    for (i, arg) in args.iter().enumerate() {
        if arg == "--interval" && i + 1 < args.len() {
            interval = args[i + 1].parse::<u64>().unwrap_or(5); // 如果解析失败，使用默认值5秒
        } else if arg == "--help" {
            print_help();
            return;
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

    let mut system = System::new_all();

    while running.load(Ordering::SeqCst) {
        system.refresh_all();

        // 检查父进程是否还活着
        match system.process(Pid::from_u32(current_pid)) {
            Some(ps) => match ps.parent() {
                Some(parent) => {
                    if parent.as_u32() == 1 {
                        println!("Parent process exited. Exiting...");
                        process::exit(0);
                    }
                }
                None => {
                    println!("Parent process exited. Exiting...");
                    process::exit(0);
                }
            },
            None => {
                println!("Parent process exited. Exiting...");
                process::exit(0);
            }
        }

        thread::sleep(Duration::from_secs(interval));

        if let Some(root) = stat::build_process_tree(&system, Pid::from_u32(target_pid)) {
            // print_process_tree(&root, 0);
            // 使用 serde_json 序列化 ProcessNode 为 JSON
            let json = serde_json::to_string(&root).unwrap();
            println!("{}", json);
        } else {
            println!("No process found with PID: {}", target_pid);
        }
    }
}
