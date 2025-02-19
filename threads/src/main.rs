use std::{thread, time::Duration};



fn main() {
    let handle=thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();

    for i in  1..5{
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));

    }   
    // handle.join().unwrap();

        //move keyword moves the one ownership of the variable to the closure

    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
   

    handle.join().unwrap();
}
