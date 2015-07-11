use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let numbers = Arc::new(Mutex::new(vec![1, 2, 3]));

    let mut threads = vec![];
    for i in 0..3 {
        let number = numbers.clone();

        let cur = thread::spawn(move|| {
            let mut array = number.lock().unwrap();

            array[i] += 1;

            println!("numbers[{}] is {}", i, array[i]);
        });
        threads.push(cur);
    }

    for i in threads {
        let _ = i.join();
    }
}
