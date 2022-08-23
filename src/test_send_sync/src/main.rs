use std::ptr::addr_of_mut;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct MyBox(*mut u8);

unsafe impl Send for MyBox {

}

unsafe impl Sync for MyBox {
    
}

fn main() {
    let p = MyBox(5 as *mut u8);
    let t = thread::spawn(move || {
        println!("{:?}", p);
    });
    t.join().unwrap();

    let b = &MyBox(5 as *mut u8);
    let v = Arc::new(Mutex::new(b));
    let t = thread::spawn(move || {
        let _v1 = v.lock().unwrap();
    });
    t.join().unwrap();
}
