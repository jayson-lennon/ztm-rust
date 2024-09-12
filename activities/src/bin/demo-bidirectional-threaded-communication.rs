use crossbeam_channel::unbounded;
use std::thread;

enum WorkerMsg {
    PrintData(String),
    Sum(i64, i64),
    Quit,
}

enum MainMsg {
    SumResult(i64),
    WorkerQuit,
}

fn main() {
    let (worker_tx, worker_rx) = unbounded();
    let (main_tx, main_rx) = unbounded();

    let worker = thread::spawn(move || loop {
        match worker_rx.recv() {
            Ok(msg) => match msg {
                WorkerMsg::PrintData(d) => println!("Worker: {}", d),
                WorkerMsg::Sum(lhs, rhs) => {
                    println!("Worker: summing...");
                    main_tx.send(MainMsg::SumResult(lhs + rhs));
                    ()
                }
                WorkerMsg::Quit => {
                    println!("Worker: terminating...");
                    main_tx.send(MainMsg::WorkerQuit);
                    break;
                }
            },
            Err(e) => {
                println!("Worker: disconnected");
                main_tx.try_send(MainMsg::WorkerQuit);
                break;
            }
        }
    });

    worker_tx.send(WorkerMsg::PrintData("hello from main!".to_owned()));
    worker_tx.send(WorkerMsg::Sum(10, 10));
    worker_tx.send(WorkerMsg::Quit);

    while let Ok(msg) = main_rx.recv() {
        match msg {
            MainMsg::SumResult(answer) => println!("Main: answer = {}", answer),
            MainMsg::WorkerQuit => println!("Main: worker terminated"),
        }
    }

    worker.join();
}
