use std::marker::PhantomPinned;
use std::mem;
use std::pin::Pin;

#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

impl Test {
    fn new(txt: &str) -> Self {
        Test {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned,
        }
    }

    fn init(self: Pin<&mut Self>) {
        let self_ref: *const String = &self.a;
        let this = unsafe { self.get_unchecked_mut() };
        this.b = self_ref;
    }

    fn a(self: Pin<&Self>) -> &str {
        &self.get_ref().a
    }

    fn b(self: Pin<&Self>) -> &String {
        assert!(!self.b.is_null(), "Test::b called without Test::init being called first");
        unsafe { &*(self.b) }
    }
}

fn main() {
    // let mut test1 = Test::new("test1");
    // test1.init();
    // let mut test2 = Test::new("test2");
    // test2.init();
    //
    // println!("a: {}, b: {}", test1.a(), test1.b());
    // std::mem::swap(&mut test1, &mut test2);
    // test1.a = "I've totally changed now!".to_string();
    // println!("a: {}, b: {}", test2.a(), test2.b())

    let mut test1 = Test::new("test1");
    let mut test1 = unsafe { Pin::new_unchecked(&mut test1) };
    Test::init(test1.as_mut());

    let mut test2 = Test::new("test2");
    let mut test2 = unsafe { Pin::new_unchecked(&mut test2) };
    Test::init(test2.as_mut());

    println!("a: {}, b: {}", Test::a(test1.as_ref()), Test::b(test1.as_ref()));
    // std::mem::swap(test1.get_mut(), test2.get_mut());
    println!("a: {}, b: {}", Test::a(test2.as_ref()), Test::b(test2.as_ref()));

    let mut test1 = Test::new("test1");
    let mut test1_pin = unsafe { Pin::new_unchecked(&mut test1) };
    Test::init(test1_pin.as_mut());

    drop(test1_pin);
    println!(r#"test1.b points to "test1": {:?}..."#, test1.b);

    let mut test2 = Test::new("test2");
    mem::swap(&mut test1, &mut test2);
    println!("... and now it points nowhere: {:?}", test1.b);

    let mut test1 = TestHeap::new("test1");
    let mut test2 = TestHeap::new("test2");

    println!("a: {}, b:{}", test1.as_ref().a(), test1.as_ref().b());
    mem::swap(&mut test1, &mut test2);
    println!("a: {}, b:{}", test2.as_ref().a(), test2.as_ref().b());
}

#[derive(Debug)]
struct TestHeap {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

impl TestHeap {
    fn new(txt: &str) -> Pin<Box<Self>> {
        let t = TestHeap {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned,
        };
        let mut boxed = Box::pin(t);
        let self_ptr: *const String = &boxed.as_ref().a;
        unsafe { boxed.as_mut().get_unchecked_mut().b = self_ptr };
        boxed
    }

    fn a(self: Pin<&Self>) -> &str {
        &self.get_ref().a
    }

    fn b(self: Pin<&Self>) -> &String {
        unsafe { &*self.b }
    }
}
