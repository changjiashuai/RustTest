use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // let (tx, rx) = mpsc::channel();
    // thread::spawn(move ||{
    //     tx.send(1).unwrap();
    // });
    // // println!("receive {}", rx.recv().unwrap());
    // println!("receive {:?}", rx.try_recv());
    //
    // let (tx, rx) = mpsc::channel();
    // thread::spawn(move ||{
    //     let s = String::from("I, flying");
    //     tx.send(s).unwrap();
    //     //value borrowed here after move.
    //     // println!("val is {}", s);
    // });

    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);
    //
    // let (tx, rx) = mpsc::channel();
    // thread::spawn(move ||{
    //     let vals = vec![
    //         String::from("hi"),
    //         String::from("from"),
    //         String::from("the"),
    //         String::from("thread"),
    //     ];
    //
    //     for val in vals {
    //         tx.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });
    //
    // for received in rx {
    //     println!("Got: {}", received);
    // }
    //
    //
    // let (tx, rx) = mpsc::channel();
    // let tx1 = tx.clone();
    // thread::spawn(move ||{
    //     tx.send(String::from("hi from raw tx")).unwrap();
    // });
    //
    // thread::spawn(move ||{
    //     tx1.send(String::from("hi from cloned tx")).unwrap();
    // });
    //
    // for received in rx {
    //     println!("Got: {}", received);
    // }
    //
    // let (tx, rx) = mpsc::sync_channel(0);
    // let handle = thread::spawn(move ||{
    //     println!("send before");
    //     tx.send(1).unwrap();
    //     println!("send after");
    // });
    //
    // println!("sleep before");
    // thread::sleep(Duration::from_secs(3));
    // println!("sleep after");
    //
    // println!("receive {}", rx.recv().unwrap());
    // handle.join().unwrap();

    let (tx, rx) = mpsc::channel();

    tx.send(Fruit::Orange("sweet".to_string())).unwrap();
    tx.send(Fruit::Apple(2)).unwrap();

    for _ in 0..2 {
        match rx.recv().unwrap() {
            Fruit::Apple(count) => println!("received {} apples", count),
            Fruit::Orange(flavor) => println!("received {} oranges", flavor),
        }
    }

    let (send, recv) = mpsc::channel();
    let num_threads = 3;
    for i in 0..num_threads {
        let thread_send = send.clone();
        thread::spawn(move ||{
            thread_send.send(i).unwrap();
            println!("thread {:?} finished", i);
        });
    }
    //
    drop(send);

    for x in recv {
        println!("Got: {}", x);
    }
    println!("finished iterating");
}

enum Fruit {
    Apple(u8),
    Orange(String)
}