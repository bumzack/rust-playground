#![feature(scoped)]

use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};

use std::fmt;
use std::fmt::{ Debug, Display, Error, Formatter };



enum ThreadStatus {
    Started (i32),
    Waiting (i32),
    Finished (i32),
}


impl Display for ThreadStatus {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &ThreadStatus::Started(n) => write!(f, "Display:   thread started:  {:?}", n),
            &ThreadStatus::Waiting(n) => write!(f, "Display:  thread waiting:   {:?}",n ),
            &ThreadStatus::Finished(n) => write!(f, "Display:  thread finished:  {:?}", n), 
        }
    }
}

impl Debug for ThreadStatus {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        fmt::Display::fmt(self, f)
    }
}


fn main () {
    // Create a shared channel that can be sent along from many threads
    // where tx is the sending half (tx for transmission), and rx is the receiving
    // half (rx for receiving).
    // let (tx, rx) = channel();
    let (tx, rx): (Sender<ThreadStatus>, Receiver<ThreadStatus>) = channel();


    let mut senderlist: Vec<Sender<ThreadStatus>> = vec![];
    // let mut allthreadstatus: Vec<ThreadStatus> = vec![]; 

    for i in 0..10 {
        let tx = tx.clone();
 
        let (tx2, rx2): (Sender<ThreadStatus>, Receiver<ThreadStatus>) = channel();
        senderlist.push(tx2);
 
        // allthreadstatus.push(threadstatus);
 
        thread::scoped(move|| {

            let mut mystatus = ThreadStatus::Started(i);
             
            if (i > 6) {
                mystatus = ThreadStatus::Finished(i);
            }

            // let dur :u32 = ((9-i) as u32)* 300 + 4500;    
            let dur :u32 = 1500;    

             println!("pause duration  {:?} ms for thread  {:?} ", dur, i);

            println!("start sleeping for {:?} ms in thread {:?}", dur, i);
            thread::sleep_ms(dur);
            println!("finished sleeping for {:?} ms in thread {:?}", dur, i);

            println!("in thread {:?}, sending status", i );
            let res_send = tx.send(mystatus);

            // let mut count = 0u32;

            //loop {
            //    mystatus = ThreadStatus::Waiting(i);
            //    let res_send = tx.send(mystatus);
             //   thread::sleep_ms(dur);

            //    count += 1;
            //    println!("in thread {:?}, count = {:?}", i, count);
            //}

            //  println!("in thread {:?}", rx2.recv().unwrap());
         });
    }

    //for k in 0..10 {
     //   let mut z = k as i32;
    //    z = z + 100;  
     //   println!("sending z = {:?} from main thread to 'spawned' thread, k = {:?}", z, k);
    //    senderlist[k].send(z).unwrap();
    //   }  

    for _ in 0..10 {
        let threadstatus = rx.recv().unwrap();
        println!("'main thread' received {:?} from spawned thread", threadstatus);

        //let threadid = match threadstatus {
        //    ThreadStatus::Started(n) => println!(f, "received Status 'Started' from thread {:?}", n),
        //    ThreadStatus::Waiting(n) => println!(f, "received Status 'Waiting' from thread {:?}", n)
        //    ThreadStatus::Finished(n) => println!(f, "received Status 'Finished' from thread {:?}", n)
       // }

       let threadid = match threadstatus {
            ThreadStatus::Started(n) => { println!("BLA BLA:   thread started:  {:?}", n); n},  
            ThreadStatus::Waiting(n) => {println!("BLA BLA:   thread Waiting:  {:?}", n); n},  
            ThreadStatus::Finished(n) => {println!("BLA BLA:   thread Finished:  {:?}", n); n},  
        };
        println!("thread id returned :   {:?}", threadid);

        // assert!(0 <= j && j < 11);
    }    
}



