use std::thread;
use std::sync::mpsc; // Multiple Producer, Single Consumer
use std::time::Duration;

pub fn threads_example() {

    // Create New Channel with Transmitter and Receiver
    let (sender, receiver) = mpsc::channel();

    let v = vec![1, 2, 3];

    let _sender1 = mpsc::Sender::clone(&sender);
    let _sender2 = mpsc::Sender::clone(&sender);

   /* - New Thread - spawn new thread "detached" from current thread that produces
    *   a `JoinHandle` that may be used to wait for completion of child thread.
    *   It will outlive the parent thread unless the parent is the main thread.
    * - Pass the `thread::spawn` function a Closure containing code to run in new thread.
    * - `Builder` Type used to configure the thread before its spawned and allows
    *   setting of the thread name and stack size for the child thread.
    * - `Thread` Type generated from spawning new thread or calling `JoinHandle`
    */
    let child_handle: thread::JoinHandle<()> = thread::Builder::new()
        .name("child1".to_string())
        .spawn(move || {
            println!("Created child thread 1! {:?}", thread::current());
            // `move` is used to transfer "ownership" of `v` from the Main Thread 
            // to the new child spawned thread
            println!("Here's a vector: {:?}", v);

            for i in 1..3 {
                println!("child1 thread: Sleep number {}", i);
                // Force thread to sleep for duration to allow different thread to run
                thread::sleep(Duration::from_millis(1));

                let val = String::from("hi");

                match _sender1.send(val) {
                    Ok(msg) => { println!("Success Transmitted child1 message {:?}", msg); },
                    Err(err) => { println!("Error Transmitting child1 message {}", err); }
                };
            }

        })
        .expect("Failed to spawn child1 thread");

//    let child_handle_two: thread::JoinHandle<()> = thread::Builder::new()
//        .name("child2".to_string())
//        .spawn(move || {
//            println!("Created child thread 2! {:?}", thread::current());
//            // `move` is used to transfer "ownership" of `v` from the Main Thread
//            // to the new child spawned thread
//            println!("Here's a vector: {:?}", v);
//
//            // Move Transmitting end of Channel `tx` into new spawned thread
//            // so child thread "owns" `tx`.
//            // Communicate from child thread to main thread by sending a message string.
//            // let val = String::from("hi");
//            let vals = vec!["hello", "from", "child", "chain"];
//
//            // `send` method returns `Result<T, E>`. If receiver already dropped it will
//            // error. Avoid using `unwrap` in a real application.
//
//            let transmitted_message = {
//                for (i, val) in vals.iter().enumerate() {
//                    match sender2.send(val) {
//                        Ok(msg) => { println!("Success Transmitted child2 message {:?}", msg); },
//                        Err(err) => { println!("Error Transmitting child2 message {}", err); }
//                    };
//
//                    println!("child2 thread: Sleep number {}", i);
//                    // Force thread to sleep for duration to allow different thread to run
//                    thread::sleep(Duration::from_millis(1));
//                }
//            };
//        })
//        .expect("Failed to spawn child2 thread");

    /* Blocks the Main Thread execution and wait until a value is sent down the channel.
     * Receipt of a sent value causes `recv` to return it in `Result<T, E>`
     * Closure of channel by sender causes `recv` to return an error indicating no more values.
     * Alternative: `try_recv` method does not block. See:
     *   https://github.com/rust-lang/book/blob/master/2018-edition/src/ch16-02-message-passing.md
     */
    let _received_message_blocking = match receiver.recv() {
        Ok(msg) => { println!("Success Receiving message {}", msg); },
        Err(err) => { println!("Error Receiving message {}", err); }
    };

    /* `join` method returns `Result` with final value produced by child thread if `Ok`
    * of `Err` if child panicked. Alternatively use `child_handle.join().unwrap();`
    */
    let _res = match child_handle.join()  {
        Ok(final_value) => println!("Final value {:?}", final_value),
        Err(error) => println!("Error {:?}", error),
    };

    // `join` on the child spawned thread blocks the main thread until the child work is complete
    for i in 1..3 {
        println!("main thread: Sleep number {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    for received in receiver {
        println!("Received {}", received);
    }

//    loop {
//        thread::sleep(Duration::from_millis(1));
//
//        let _received_message_non_blocking = match receiver.try_recv() {
//            Ok(msg) => { println!("Success Receiving message non-blocking {}", msg); true }
//            Err(mpsc::TryRecvError::Empty) => {
//                drop(receiver);
//                drop(&child_handle);
//                println!("Dropping Receiver as message non-blocking took too long");
//                false
//            },
//            Err(mpsc::TryRecvError::Disconnected) => {
//                println!("Error unreachable message non-blocking");
//                unreachable!()
//            }
//        };
//    }
}