use std::ops::{Deref, DerefMut};
use std::rc::Rc;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Self {
        Person { name, age }
    }

    fn display(self: &mut Person, age: u8) {
        let Person { name, age } = &self;
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

struct Foo;

impl Foo {
    fn foo(&self) {
        println!("Foo");
    }
}

fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // let person = Person::new("hello".to_string(), 18);
    // println!("{}", *person);

    let x = Box::new(1);
    let sum = *x + 1;
    assert_eq!(sum, 2);

    let y = MyBox::new(5);
    assert_eq!(5, *y);
    assert_eq!(5, *(y.deref()));

    let s = String::from("hello world");
    display(&s);

    let s = MyBox::new(String::from("hello world"));
    display(&s);
    display(&s[..]);

    let m = MyBox::new(String::from("Rust"));
    display(&(*m));
    display(&(*m)[..]);
    display(&(*m).deref());

    let s = MyBox::new(String::from("hello world"));
    let s1: &str = &s;
    let s2: String = s.to_string();

    let owned = "Hello".to_string();
    let counted = Rc::new(owned);
    display(&counted);

    let f = &&Foo;
    f.foo();
    (&f).foo();
    (&&f).foo();
    (*f).foo();
    (**f).foo();
    // (***f); need impl deref

    let mut s = MyBox::new(String::from("hello, "));
    display(&s);
    display2(&mut s);
    display(&s);
}

fn display2(s: &mut String){
    s.push_str("world");
    println!("{}", s);
}

fn display(s: &str) {
    println!("{}", s);
}
