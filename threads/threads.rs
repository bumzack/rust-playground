#![feature(scoped)]

use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};

use std::fmt;
use std::fmt::{ Debug, Display, Error, Formatter};


enum ThreadStatus {
    Start (i32),
    Stopp (i32),
    Waiting (i32),
    Finished (i32),
}


impl Display for ThreadStatus {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &ThreadStatus::Start(n) => write!(f, "thread Start:  id = {:?}", n),
            &ThreadStatus::Waiting(n) => write!(f, "thread waiting:  id =  {:?}",n ),
            &ThreadStatus::Finished(n) => write!(f, "thread finished:  id = {:?}", n), 
            &ThreadStatus::Finished(n) => write!(f, "thread finished:  id = {:?}", n),
            &ThreadStatus::Stopp(n) => write!(f, "thread stopp:  id = {:?}", n), 
        }
    }
}

impl Debug for ThreadStatus {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        fmt::Display::fmt(self, f)
    }
}


static NCORES: i32 = 3;



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

            let threadid = i; 

            let mut mystatus = ThreadStatus::Start(i);
             
            if (threadid > 6) {
                mystatus = ThreadStatus::Waiting(i);
            }

            // let dur :u32 = ((9-threadid) as u32)* 300 + 4500;    
            let dur :u32 = 1500;    

             println!("pause duration  {:?} ms for thread  {:?} ", dur, threadid);

            println!("start sleeping for {:?} ms in thread {:?}", dur, threadid);
            thread::sleep_ms(dur);
            println!("finished sleeping for {:?} ms in thread {:?}", dur, threadid);

            println!("in thread {:?}, sending status", threadid);
            let res_send = tx.send(mystatus);

            let mut count = 0i32;

            loop {
                 // do some work (wait ...)
 
                println!("THREAD LOOP        start sleeping for {:?} ms in thread {:?}", dur, i);
                thread::sleep_ms(dur);
                println!("THREAD LOOP      finished sleeping for {:?} ms in thread {:?}", dur, i);
                count += 1;
                println!("in thread {:?}, count = {:?}", i, count);

                // tell the main thread -> i'm done and waiting for new things to do 
                mystatus = ThreadStatus::Waiting(threadid);
                let res_send = tx.send(mystatus);

                // wait for the response of the main thread -> there are only 2 valid responses
                //  1) gimme some new work to do    ("Start")
                //  2) tell me to shut down         ("Stopp")

                let new_data = rx2.recv().unwrap();

                let number = match new_data {
                    ThreadStatus::Start(n) => { 
                                                    println!("SUB THREAD in scoped thread - main thread tells me to start;  {:?} from main thread ", n); 
                                                    n
                                                }, 
                    ThreadStatus::Stopp(n) => { 
                                                    println!("SUB THREAD in scoped thread - main thread tells me to stopp;  {:?} from main thread "); 
                                                    mystatus = ThreadStatus::Finished(threadid);
                                                    let res_send = tx.send(mystatus);
                                                    return;
                                            }, 
                    _ => panic!("WHOA - something went terrible wrong. never ever send something different than 'stop' oder 'start' to a thread"),  
                };
                println!("got number from main thread  :   {:?}", number); 
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
        println!("'MAIN THREAD' received {:?} from spawned thread", threadstatus);
 
       let threadid :i32 = match threadstatus {
            ThreadStatus::Start(n) => { println!("'MAIN THREAD':  Start number:  {:?}", n); n},  
            ThreadStatus::Waiting(n) => {println!("'MAIN THREAD': waiting  number:  {:?}", n); n},  
            ThreadStatus::Finished(n) => { 
                                            println!("'MAIN THREAD': finished  number:   {:?}", n); 
                                            count_threads_running -= 1; 
                                            n
                                          },  
        };
        println!("'MAIN THREAD'   worker thread with id returned :   {:?}", threadid);

        let sendnumber  = ThreadStatus::Start(count);
        count += 1;

        let bla = threadid as usize; 
        println!("MAIN THREAD -  sending data to thread id :  {:?}", bla);
        let res_send = &senderlist[bla];
        res_send.send(sendnumber);

        if (count_threads_running == 0) {
            break; 
        }
    }    
    println!("'MAIN THREAD' finished 'loop'");

}
