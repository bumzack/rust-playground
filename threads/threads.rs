use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};

enum ThreadStatus {
    Started (i32),
    Waiting (i32),
    Finished (i32),
}


fn main () {
    // Create a shared channel that can be sent along from many threads
    // where tx is the sending half (tx for transmission), and rx is the receiving
    // half (rx for receiving).
    let (tx, rx) = channel();
 

    let mut senderlist: Vec<Sender<i32>> = vec![];
    // let mut allthreadstatus: Vec<ThreadStatus> = vec![]; 

    for i in 0..10 {
        let tx = tx.clone();
 
        let (tx2, rx2): (Sender<i32>, Receiver<i32>) = channel();
        senderlist.push(tx2);

        let mystatus = ThreadStatus::Started(i);

        // allthreadstatus.push(threadstatus);
 
        thread::spawn(move|| {

            let dur :u32 = (i as u32)* 800 + 1500;    
            println!("pause duration  {:?} ms for thread  {:?} ", dur, i);

            println!("start sleeping for {:?} ms in thread {:?}", dur, i);
            thread::sleep_ms(dur);
            println!("finished sleeping for {:?} ms in thread {:?}", dur, i);

            tx.send(i+1).unwrap();
            println!("in thread {:?}", rx2.recv().unwrap());
         });
    }

    for k in 0..10 {
        let mut z = k as i32;
        z = z + 100;  
        println!("sending z = {:?} from main thread to 'spawned' thread, k = {:?}", z, k);
        senderlist[k].send(z).unwrap();
    }  

    for _ in 0..10 {
        let j = rx.recv().unwrap();
        println!("'main thread' received {:?} from spawned thread", j);

        assert!(0 <= j && j < 11);
    }    
}



