// #![feature(once_cell)]
#![feature(once_cell)]
use std::lazy::SyncOnceCell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

static mut REQUEST_RECV: usize = 0;

static REQUEST_RECV_AC: AtomicUsize = AtomicUsize::new(0);

// static NAMES: Mutex<String> = Mutex::new(String::from("a, o, e, i"));

#[derive(Debug)]
struct Config {
    a: String,
    b:String,
}
static mut CONFIG: Option<&mut Config> = None;

fn init() -> Option<&'static mut Config> {
    // Some(&mut Config{
    //     a: "A".to_string(),
    //     b: "B".to_string(),
    // })
    let c = Box::new(Config{
        a: "A".to_string(),
        b: "B".to_string(),
    });
    Some(Box::leak(c))
}

#[derive(Debug)]
struct  Logger;

static LOGGER: SyncOnceCell<Logger> = SyncOnceCell::new();

fn main() {
    unsafe {
        REQUEST_RECV += 1;
        assert_eq!(REQUEST_RECV, 1);
    }

    for _ in 0..100 {
        REQUEST_RECV_AC.fetch_add(1, Ordering::Relaxed);
    }
    println!("cur i = {:?}", REQUEST_RECV_AC);

    // let v = NAMES.lock().unwrap();
    // println!("{}", v);

    // unsafe {
    //     CONFIG = Some(&mut Config{
    //         a: "A".to_string(),
    //         b: "B".to_string(),
    //     });
    //     println!("{:?}", CONFIG);
    // }

    let c = Box::new(Config{
        a: "A".to_string(),
        b: "B".to_string(),
    });

    unsafe {
        // CONFIG = Some(Box::leak(c));
        CONFIG = init();
        println!("{:?}", CONFIG);
    }
}
