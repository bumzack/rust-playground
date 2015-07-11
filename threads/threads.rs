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


static NCORES: i32 = 4;



fn main () {
    // Create a shared channel that can be sent along from many threads
    // where tx is the sending half (tx for transmission), and rx is the receiving
    // half (rx for receiving).
    // let (tx, rx) = channel();
    let (tx, rx): (Sender<ThreadStatus>, Receiver<ThreadStatus>) = channel();


    let mut senderlist: Vec<Sender<ThreadStatus>> = vec![];
    // let mut allthreadstatus: Vec<ThreadStatus> = vec![]; 


    let mut handles = Vec::new();


    let mut count_threads_running = 0; 

    for i in 0..NCORES {
        let tx = tx.clone();
 
        let (tx2, rx2): (Sender<ThreadStatus>, Receiver<ThreadStatus>) = channel();
        senderlist.push(tx2);
 
        // allthreadstatus.push(threadstatus);
 
        let worker = thread::scoped(move|| {

            let mut mystatus = ThreadStatus::Started(i);
             
            if (i > 6) {
                mystatus = ThreadStatus::Waiting(i);
            }

            // let dur :u32 = ((9-i) as u32)* 300 + 4500;    
            let dur :u32 = 1500;    

             println!("pause duration  {:?} ms for thread  {:?} ", dur, i);

            println!("start sleeping for {:?} ms in thread {:?}", dur, i);
            thread::sleep_ms(dur);
            println!("finished sleeping for {:?} ms in thread {:?}", dur, i);

            println!("in thread {:?}, sending status", i );
            let res_send = tx.send(mystatus);

            let mut count = 0i32;

            loop {
                mystatus = ThreadStatus::Waiting(i);
                let res_send = tx.send(mystatus);
 
                println!("THREAD LOOP        start sleeping for {:?} ms in thread {:?}", dur, i);
                thread::sleep_ms(dur);
                println!("THREAD LOOP      finished sleeping for {:?} ms in thread {:?}", dur, i);
                count += 1;
                println!("in thread {:?}, count = {:?}", i, count);

                let new_data = rx2.recv().unwrap();

                let number = match new_data {
                    ThreadStatus::Started(n) => { println!("SUB THREAD   in scoped thread - got  {:?} from main thread ", n); n}, 
                    ThreadStatus::Waiting(n) => { println!("SUB THREAD  in scoped thread - got  {:?} from main thread ", n); n}, 
                    ThreadStatus::Finished(n) => { println!("SUB THREAD  in scoped thread - got  {:?} from main thread ", n); n},  
                };
                println!("got number from main thread  :   {:?}", number);
  
                if count > 6 {
                    mystatus = ThreadStatus::Finished(count + i*100);
                    let res_send = tx.send(mystatus);
                    return;
                } 
            }

            //  println!("in thread {:?}", rx2.recv().unwrap());
         });

         handles.push(worker);
         count_threads_running += 1;
    }

    //for k in 0..NCORES {
     //   let mut z = k as i32;
    //    z = z + 100;  
     //   println!("sending z = {:?} from main thread to 'spawned' thread, k = {:?}", z, k);
    //    senderlist[k].send(z).unwrap();
    //   }  

    let mut count = 10000; 
    loop 
     {
        let threadstatus = rx.recv().unwrap();
        println!("'main thread' received {:?} from spawned thread", threadstatus);
 
       let threadid :i32 = match threadstatus {
            ThreadStatus::Started(n) => { println!("main thread:  started number:  {:?}", n); n},  
            ThreadStatus::Waiting(n) => {println!("main thread: waiting  number:  {:?}", n); n},  
            ThreadStatus::Finished(n) => { 
                                            println!("main thread: finished  number:   {:?}", n); 
                                            count_threads_running -= 1; 
                                            n
                                          },  
        };
        println!("thread id returned :   {:?}", threadid);

        let sendnumber  = ThreadStatus::Started(count);
        count += 1;

        println!("MAIN THREAD - sending to sub thread :");
        let bla = threadid as usize; 
        println!("MAIN THREAD -  sending data to thread id :  {:?}", bla);
        let res_send = &senderlist[bla];
        res_send.send(sendnumber);

        // assert!(0 <= j && j < 11);
        if (count_threads_running == 0) {
            break; 
        }
    }    
}
