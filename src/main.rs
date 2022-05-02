use std::{env, thread};
use std::process::exit;
use std::sync::mpsc::channel;
use ip_sniffer::{Arguments, scan, SnifferError};

fn main() {
    let args = &mut env::args();
    let program = &args.next().unwrap();
    let args = args.collect::<Vec<String>>();
    let arguments = Arguments::new(&args).unwrap_or_else(|e| {
        match e {
            SnifferError::Help => {}
            e => {
                eprintln!("{} error: {}", program, e);
            }
        }
        exit(1);
    });
    let addr = arguments.ip_addr();
    let threads = arguments.num_threads();
    let (tx, rx) = channel::<i32>();
    for thread in 1..threads {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            scan(thread, threads, tx_clone, addr);
        });
    }

    drop(tx);

    let mut ports = vec![];
    for port in rx {
        ports.push(port);
    }

    println!("Done!");

    for port in ports {
        println!("Port {} is open", port);
    }
}
