use learn_base_about_threads::learn;
mod learn_base_about_threads {
    use std::thread;
    use std::time::Duration;

    pub fn learn() {
        let handle = thread::spawn(move || {
            for i in 1..10 {
                println!("hi number {i} from the spawned thread!");
                thread::sleep(Duration::from_secs(1));
            }
        });

        // the main thraid waits when thraid 'handle' end
        handle.join().unwrap();

        for i in 1..5 {
            println!("hi number {i} from the main thread!");
            thread::sleep(Duration::from_secs(1));
        }

        // handle.join().unwrap();
    }
}
use learn_threads_channels::{base, multi_threads_resp};
mod learn_threads_channels {
    use std::time::Duration;
    use std::{sync::mpsc, thread};

    pub fn base() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let resp = String::from("hello");

            tx.send(resp).unwrap();
        });

        let recv = rx.recv().unwrap();

        println!("{recv:?}");
    }
    pub fn multi_threads_resp() {
        let (tx, rx) = mpsc::channel();
        let tx2 = tx.clone();

        let t = thread::spawn(move || {
            let resp = vec!["hello", "from", "spawned", "thread", "myav"];

            for r in resp {
                thread::sleep(Duration::from_secs(1));
                tx2.send(r).expect("msg");
            }
        });
        t.join().unwrap();

        thread::spawn(move || {
            let mut resp = vec!["wow", "second", "thread", "!"];

            for i in 0..resp.len() {
                thread::sleep(Duration::from_secs(1));

                tx.send(resp[i]).unwrap();
            }
        });

        let mut messages = String::new();
        for i in rx {
            messages.push_str(format!("{i}\n").as_str());

            println!("recv = {i}🇷🇺💖🇺🇦");
        }
    }
}

use learn_mutex::{base_about_mutex, deadlock, multi_threads_mutex};
mod learn_mutex {
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    pub fn base_about_mutex() {
        let m = Mutex::new(1);

        {
            let mut num = m.lock().unwrap();

            *num += 7;
        }

        println!("{m:?}");
    }
    pub fn multi_threads_mutex() {
        let mut count = Arc::new(Mutex::new(0));
        let mut handles = Vec::new();

        for _ in 0..10 {
            let count = Arc::clone(&count);

            let handle = thread::spawn(move || {
                let mut num = count.lock().unwrap();

                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *count.lock().unwrap());
    }

    // to see how fix deadlock uncomment lines from 140 to 146 and delete
    // 'data2_clone.lock().unwrap()' in line 139
    pub fn deadlock() {
        let data1 = Arc::new(Mutex::new(0));
        let data2 = Arc::new(Mutex::new(10));

        // create first thread
        let thread1 = thread::spawn({
            // clone data1 and data2
            let data1_clone = Arc::clone(&data1);
            let data2_clone = Arc::clone(&data2);

            move || {
                // lock data1
                let mut guard = data1_clone.lock().unwrap();
                // increment data in data1
                *guard += 1;
                println!("thread1 g1 = {guard}");

                // little sleep to start thread2
                thread::sleep(Duration::from_secs(1));

                // oops! data2 was locked by thread2!
                // thread1 will wait until data2 will be unlocked, but data2 will never be unlocked!
                let mut guard2 = data2_clone.lock().unwrap();

                // match data2_clone.try_lock() {
                //     Ok(mut m) => {
                //         *m += 1;
                //         println!("thread1 g2 = {m}");
                //     }
                //     Err(e) => println!("thread1 err = {e}"),
                // };
            }
        });

        // create second thread
        let thread2 = thread::spawn({
            let data1_clone = Arc::clone(&data1);
            let data2_clone = Arc::clone(&data2);

            move || {
                // lock data2
                let mut guard2 = data2_clone.lock().unwrap();
                *guard2 += 1;
                println!("thread2 g2 = {guard2}");

                thread::sleep(Duration::from_secs(1));

                // oops! data1 was locked by thread1!
                // thread2 will wait until data1 will be unlocked, but data1 will never be unlocked!
                let mut guard1 = data1_clone.lock().unwrap();
                *guard1 += 1;
            }
        });

        thread1.join().unwrap();
        thread2.join().unwrap();

        println!("data1 = {:?}, data2 = {:?}", data1, data2);
    }
}
fn main() {
    // 1. Base about threads (learn_base_about_threads module)
    //
    // learn();

    // 2. Passing values ​​between threads (learn_threads_channels module)
    //
    // base();
    // multi_threads_resp();

    // 3. Shared-State Concurrency (learn_mutex module)
    //
    // base_about_mutex();
    // multi_threads_mutex();
    // deadlock();
}
