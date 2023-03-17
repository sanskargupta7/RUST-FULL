//Concurrent Programming

//Using Threads to Run Code Simultaneously

///In most current operating systems, an executed program’s code is run in a process, and the operating system will manage multiple processes at once. 
/// Within a program, you can also have independent parts that run simultaneously. 
/// The features that run these independent parts are called threads. 
/// For example, a web server could have multiple threads so that it could respond to more than one request at the same time.


//Because threads can run simultaneously, there’s no inherent guarantee about the order in which parts of your code on different threads will run. 
//This can lead to problems, such as:

//1 - Race conditions, where threads are accessing data or resources in an inconsistent order
//2 - Deadlocks, where two threads are waiting for each other, preventing both threads from continuing
//3 - Bugs that happen only in certain situations and are hard to reproduce and fix reliably

//-------------------------------------------------------------------------------------

//Creating a New Thread with spawn

//To create a new thread, we call the thread::spawn function and pass it a closure (we talked about closures in Chapter 13) containing the code we want to run in the new thread.

//thread::sleep(Duration::from_millis(1));
//we can use the statement above in order to make our thread sleep for a certain amout of time

use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {                                           // we spawned a thread and put the code for the thread inside a closure
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));               //with this we can sleep and pause the exection of out code for a certain amount of time
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

//OUTPUT IS ___

// hi number 1 from the main thread!
// hi number 1 from the spawned thread!
// hi number 2 from the main thread!
// hi number 2 from the spawned thread!
// hi number 3 from the main thread!
// hi number 3 from the spawned thread!
// hi number 4 from the main thread!
// hi number 4 from the spawned thread!

//The calls to thread::sleep force a thread to stop its execution for a short duration, allowing a different thread to run. 
//The threads will probably take turns, but that isn’t guaranteed: it depends on how your operating system schedules the threads. 
//In this run, the main thread printed first, even though the print statement from the spawned thread appears first in the code. 


//The code above not only stops the spawned thread prematurely most of the time due to the main thread ending, 
//but because there is no guarantee on the order in which threads run, we also can’t guarantee that the spawned thread will get to run at all!

//We can fix the problem of the spawned thread not running or ending prematurely by saving the return value of thread::spawn in a variable. 
//The return type of thread::spawn is JoinHandle. A JoinHandle is an owned value that, when we call the join method on it, will wait for its thread to finish.

let handle = thread::spawn(|| {                 //we stored the return of thread::spawn inside a variable
    for i in 1..10 {
        println!("hi number {} from the spawned thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
});

for i in 1..5 {
    println!("hi number {} from the main thread!", i);
    thread::sleep(Duration::from_millis(1));
}

handle.join().unwrap();                 //now join() method waits for its thread to finish......we used unwrap() because join() method returns a result and thus can return an Err variant

//The two threads continue alternating, but the main thread waits because of the call to handle.join() and does not end until the spawned thread is finished.

//handle.join().unwrap(); ...............if we would have moved this statement before the for loop in 'main'
//then that would have let the thread run completely and then moved on to the loop in the next line 
//This way we could have run the thread we spawned first....


//=============================================================

//Using move Closures with Threads

//We'll often use the move keyword with closures passed to thread::spawn 
//because the closure will then take ownership of the values it uses from the environment, 
//thus transferring ownership of those values from one thread to another.

//---------------------------------------------------

//notice that in the closure ....we are not using any value from the 'main' part

use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

//This code will not compile because we are taking 'v' which we declared outside the spawn() inside the closure....
//in order to take in the data outside...the spawn() function closure would require to capture the data from the outside....
//in the above code we captured the vector 'v' but the code still didnt run....WHY?

/// AS println! only requires the reference of 'v' the closure tries to borrow 'v' 
/// BUT THE PROBLEM - Rust can’t tell how long the spawned thread will run, so it doesn’t know if the reference to v will always be valid.

//By adding the move keyword before the closure, we force the closure to take ownership of the values it’s using rather than allowing Rust to infer that it should borrow the values.

use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {            //we added 'mpve' which made the thread to take the ownership of the vector 'v' that we created.....
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

//------------------------------------------------------------------

//Using MESSAGE PASSING to Transfer Data Between Threads

//To accomplish message-sending concurrency, Rust's standard library provides an implementation of channels. 
//A channel is a general programming concept by which data is sent from one thread to another.

//You can imagine a channel in programming as being like a directional channel of water, such as a stream or a river. 
//If you put something like a rubber duck into a river, it will travel downstream to the end of the waterway.

//A channel has two haves: a transmitter and a receiver.
//One part of your code calls methods on the transmitter with the data you want to send, and another part checks the receiving end for arriving messages. 
//A channel is said to be closed if either the transmitter or receiver half is dropped.


//this is how we create a CHANNEL
use std::sync::mpsc  ;

fn main() {
    let (tx, rx) = mpsc::channel();
}

//mpsc stands for multiple producer, single consumer. 
//the way Rust’s standard library implements channels means a channel can have multiple sending ends that produce values but only one receiving end that consumes those values. 
//The mpsc::channel function returns a tuple, the first element of which is the sending end--the transmitter--and the second element is the receiving end--the receiver. 

//Let’s move the transmitting end into a spawned thread and have it send one string so the spawned thread is communicating with the main thread

use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
}

//The spawned thread needs to own the transmitter to be able to send messages through the channel. 
//The transmitter has a send method that takes the value we want to send. 
//The send method returns a Result<T, E> type, 
//so if the receiver has already been dropped and there’s nowhere to send a value, the send operation will return an error. 
//In this example, we’re calling unwrap to panic in case of an error.

//we’ll get the value from the receiver in the main thread

let received = rx.recv().unwrap();              //as recv() and send() method return Result enum....we use the unwra to take out the value tha we want to get tanj
println!("Got: {}", received);

//The receiver has two useful methods: recv and try_recv
// 1- We’re using recv, short for receive, which will block the main thread’s execution and wait until a value is sent down the channel.
//Once a value is sent, recv will return it in a Result<T, E>. When the transmitter closes, recv will return an error to signal that no more values will be coming.
//Once a value is sent, recv will return it in a Result<T, E>. When the transmitter closes, recv will return an error to signal that no more values will be coming.

//2- The try_recv method doesn’t block, but will instead return a Result<T, E> immediately: an Ok value holding a message if one is available and an Err value if there aren’t any messages this time. 
//Using try_recv is useful if this thread has other work to do while waiting for messages: 
//we could write a loop that calls try_recv every so often, handles a message if one is available, and otherwise does other work for a little while until checking again.

//----------------------------------------------------------------------

//once the value has been sent to another thread, that thread could modify or drop it before we try to use the value again.

//The send function takes ownership of its parameter, and when the value is moved, the receiver takes ownership of it. 
//This stops us from accidentally using the value again after sending it; the ownership system checks that everything is okay.



//Sending Multiple Values and Seeing the Receiver Waiting


//The spawned thread will now send multiple messages and pause for a second between each message.

let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
    ];

    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

for received in rx {
    println!("Got: {}", received);
}


/////////////////////////////////////////////////
/// 
/// 


//Creating Multiple Producers by Cloning the Transmitter

let tx1 = tx.clone(); //just write this and you got a new producer

//now this new producer will be able to send messages using the send function

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // --snip--

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    // --snip--
}

//we call clone on the transmitter. 
//This will give us a new transmitter we can pass to the first spawned thread. 
//We pass the original transmitter to a second spawned thread. 
//This gives us two threads, each sending different messages to the one receiver.

//OUTPUT
// Got: hi
// Got: more
// Got: from
// Got: messages
// Got: for
// Got: the
// Got: thread
// Got: you

//------------------------------------------------------------------------

//Shared- State Concurrency

//Message passing is a fine way of handling concurrency, but it’s not the only one. 
//Another method would be for multiple threads to access the same shared data.

//channels in any programming language are similar to single ownership, because once you transfer a value down a channel, you should no longer use that value. 
//Shared memory concurrency is like multiple ownership: multiple threads can access the same memory location at the same time.


//Using Mutexes to Allow Access to Data from One Thread at a Time

//Mutex is an abbreviation for mutual exclusion, as in, a mutex allows only one thread to access some data at any given time. 

//To access the data in a mutex, a thread must first signal that it wants access by asking to acquire the mutex’s lock. 
//The lock is a data structure that is part of the mutex that keeps track of who currently has exclusive access to the data.

//Therefore, the mutex is described as guarding the data it holds via the locking system.

//Mutexes have a reputation for being difficult to use because you have to remember two rules:

//1 - You must attempt to acquire the lock before using the data.
//2 - When you’re done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.


//---------------------------------------------------------------------------------------------------------------------

// API for Mutex<T>

use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);                  //created a Mutex<T> with new() function
    {
        let mut num = m.lock().unwrap();       //we used lock() methopd to acquire the lock in order to access the data
        *num = 6;
    }
    println!("m = {:?}", m);
}

//OUTPUT
//m = Mutex { data: 6, poisoned: false, .. }

//To access the data inside the mutex, we use the lock method to acquire the lock. 
//This call will block the current thread so it can’t do any work until it’s our turn to have the lock.

//The call to lock would fail if another thread holding the lock panicked
//In that case, no one would ever be able to get the lock, so we’ve chosen to unwrap and have this thread panic if we’re in that situation.

//After we’ve acquired the lock, we can treat the return value, named num in this case, as a mutable reference to the data inside. 
//The type of m is Mutex<i32>, not i32, so we must call lock to be able to use the i32 value. We can’t forget; the type system won’t let us access the inner i32 otherwise.
//The type system ensures that we acquire a lock before using the value in m

//Mutex<T> is a smart pointer. 
//More accurately, the call to lock returns a smart pointer called MutexGuard, wrapped in a LockResult that we handled with the call to unwrap. 
//The MutexGuard smart pointer implements Deref to point at our inner data; 
//the smart pointer also has a Drop implementation that releases the lock automatically when a MutexGuard goes out of scope, which happens at the end of the inner scope. 
//As a result, we don’t risk forgetting to release the lock and blocking the mutex from being used by other threads, because the lock release happens automatically.

//After dropping the lock, we can print the mutex value and see that we were able to change the inner i32 to 6.
//-----------------------------------------------------------------------


//Sharing a Mutex<T> Between Multiple Threads

//lets see what happens with mutiple thread
//we make 10 threads each thread increaasing the value by 1

let counter = Mutex::new(0);
let mut handles = vec![];

for _ in 0..10 {
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();

        *num += 1;
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}

println!("Result: {}", *counter.lock().unwrap());


//gives a compile error

///WHAT WE THOUGHT
/// We create a counter variable to hold an i32 inside a Mutex<T>, as we did in Listing 16-12. Next, we create 10 threads by iterating over a range of numbers. We use thread::spawn and give all the threads the same closure: one that moves the counter into the thread, acquires a lock on the Mutex<T> by calling the lock method, and then adds 1 to the value in the mutex. When a thread finishes running its closure, num will go out of scope and release the lock so another thread can acquire it.
///In the main thread, we collect all the join handles. Then, as we did in Listing 16-2, we call join on each handle to make sure all the threads finish. At that point, the main thread will acquire the lock and print the result of this program.

//What Happened ...WHY?
/// doesnt comppile because : 
/// The error message states that the counter value was moved in the previous iteration of the loop. 
/// Rust is telling us that we can’t move the ownership of lock counter into multiple threads.

//The previous program failed becuae we cannot move or transfer the ownership of an outside variable into multiple threads.

//LETS SOLVE:

//Multiple ownership with Multiple threads

//for multiple ownership what we can do is wrap our Mutex<T> inside an Rc<T>
//then we can clone the Rc<T> before moving ownership to the thread..

let counter = Rc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Rc::clone(&counter);          //before going into a thread we wrapped the counter reference into Rc<> clone
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();

        *num += 1;
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}

println!("Result: {}", *counter.lock().unwrap());

//THIS WILL NOT COMPILE AGAIN...WHY???
//Unfortunately, Rc<T> is not safe to share across threads. 
//When Rc<T> manages the reference count, it adds to the count for each call to clone and subtracts from the count when each clone is dropped. 
//But it doesn’t use any concurrency primitives to make sure that changes to the count can’t be interrupted by another thread. 
//This could lead to wrong counts—subtle bugs that could in turn lead to memory leaks or a value being dropped before we’re done with it.

//LETS SOLVE FURTHER:

//Atomic Reference Counting with Arc<T>   

/// Arc<T> is a type like Rc<T> that is safe to use in concurrent situations.
/// it’s an atomically reference counted type.
/// Just need to know that atomics work like primitive types but are safe to share across threads.


//THREAD SAFETY comes with a Performance penalty ......and so we want to use Arc<T> only when required....hence it is not include in the standard library...
//Same is the reason for all primitive datatypes not being ATOMIC

//So just change the Rc<T> in the above program into Arc<T> and you are good to go!!!!!

//The program would compile with OUTPUT
//Result: 10

//---------------------------------------------------------------------------

//any type T is Sync if &T (an immutable reference to T) is Send, meaning the reference can be sent safely to another thread. 
//Similar to Send, primitive types are Sync, and types composed entirely of types that are Sync are also Sync.

//The smart pointer Mutex<T> is Sync and can be used to share access with multiple threads 













