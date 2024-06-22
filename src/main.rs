use libc;
use std::env;
use std::process;
use std::thread;
use std::time;
fn main() {
    let base: Vec<String> = env::args().collect();
    let args = &base[1..];
    if args.len() == 0 {
        println!("usage {} <program to run>", base[0]);
        process::exit(1);
    }
    let child = if args.len() > 1 {
        process::Command::new(args[0].clone())
            .args(&args[1..])
            .spawn()
            .expect("program doesn't exist or cannot be run")
    } else {
        process::Command::new(args[0].clone())
            .spawn()
            .expect("program doesn't exist or cannot be run\n")
    };
    let pid = child.id();
    println!("{}", pid);
    loop {
        let running = unsafe { libc::kill(pid as i32, 0) == 0 };
        if !running {
            break;
        }
        unsafe {
            let b = libc::kill(pid as i32, libc::SIGSTOP);
            if b == -1 {
                println!("failed\n");
            }
        }
        thread::sleep(time::Duration::new(0, 10_000_000));
        unsafe {
            let b = libc::kill(pid as i32, libc::SIGCONT);
            if b == -1 {
                println!("failed\n");
            }
        }
        thread::sleep(time::Duration::new(0, 10_000_000));
    }
}
