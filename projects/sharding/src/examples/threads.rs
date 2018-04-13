use std::thread;
use std::time::Duration;

pub fn threads_example() {

    let v = vec![1, 2, 3];

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
            println!("Created child thread! {:?}", thread::current());
            // `move` is used to transfer "ownership" of `v` from the Main Thread 
            // to the new child spawned thread
            println!("Here's a vector: {:?}", v);
            for i in 1..3 {
                println!("child1 thread: Sleep number {}", i);
                // Force thread to sleep for duration to allow different thread to run
                thread::sleep(Duration::from_millis(1));
            }
        })
        .expect("Failed to spawn child1 thread");

    /* `join` method returns `Result` with final value produced by child thread if `Ok`
    * of `Err` if child panicked. Alternatively use `child_handle.join().unwrap();`
    */
    let res = match child_handle.join()  {
        Ok(final_value) => println!("Final value {:?}", final_value),
        Err(error) => println!("Error {:?}", error),
    };

    // `join` on the child spawned thread blocks the main thread until the child work is complete
    for i in 1..3 {
        println!("main thread: Sleep number {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}