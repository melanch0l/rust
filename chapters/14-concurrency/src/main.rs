use std::sync::mpsc;
use std::thread;

fn main() {
    let (sender, receiver) = mpsc::channel();
    let worker = thread::spawn(move || {
        sender
            .send(String::from("message from worker"))
            .expect("receiver exists");
    });

    println!("{}", receiver.recv().expect("worker sent a message"));
    worker.join().expect("worker thread panicked");

    let total = thread::scope(|scope| {
        let left = scope.spawn(|| 20);
        let right = scope.spawn(|| 22);
        left.join().expect("left worker panicked") + right.join().expect("right worker panicked")
    });
    println!("scoped thread total: {total}");
}
