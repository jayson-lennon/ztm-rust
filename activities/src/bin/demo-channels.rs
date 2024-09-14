#![allow(dead_code)]
#![allow(unused_must_use)]

use crossbeam_channel::unbounded;
use std::thread;

enum ThreadMsg {
    PrintData(String),
    Sum(i64, i64),
    Quit,
}

fn main() {
    let (s, r) = unbounded();

    let handle = thread::spawn(move || loop {
        match r.recv() {
            Ok(msg) => match msg {
                ThreadMsg::PrintData(d) => println!("{}", d),
                ThreadMsg::Sum(lhs, rhs) => println!("{}+{}={}", lhs, rhs, lhs + rhs),
                ThreadMsg::Quit => {
                    println!("thread terminating...");
                    break;
                }
            },
            Err(_) => {
                println!("disconnected");
                break;
            }
        }
    });

    s.send(ThreadMsg::PrintData("hello from main!".to_owned()));
    s.send(ThreadMsg::Sum(10, 10));
    drop(s);
    // s.send(ThreadMsg::Quit);
    handle.join();
}
