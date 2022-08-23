use std::rc::Rc;
use std::sync::Arc;
use std::thread;

struct Owner {
    name: String,
}

struct Gadget {
    id: i32,
    owner: Rc<Owner>,
}

fn main() {
    // let s = String::from("hello, world");
    // let a = Box::new(s);
    // let b = Box::new(s);

    let a = Rc::new(String::from("hello, world"));
    let b = Rc::clone(&a);

    assert_eq!(2, Rc::strong_count(&a));
    assert_eq!(Rc::strong_count(&a), Rc::strong_count(&b));

    let a = Rc::new(String::from("test ref counting"));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Rc::clone(&a);
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Rc::clone(&a);
        println!("count after creating c = {}", Rc::strong_count(&c));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    let gadget_owner: Rc<Owner> = Rc::new(Owner {
        name: "Gadget Man".to_string()
    });

    let gadget1 = Gadget {
        id: 1,
        owner: Rc::clone(&gadget_owner),
    };

    let gadget2 = Gadget {
        id: 2,
        owner: Rc::clone(&gadget_owner),
    };

    println!("drop before gadget owner rf count = {}", Rc::strong_count(&gadget_owner));
    drop(gadget_owner);
    println!("drop after gadget1 owner rf count = {}", Rc::strong_count(&gadget1.owner));
    println!("drop after gadget2 owner rf count = {}", Rc::strong_count(&gadget2.owner));

    println!("Gadget {} owned by {}", gadget1.id, gadget1.owner.name);
    println!("Gadget {} owned by {}", gadget2.id, gadget2.owner.name);

    // let s = Rc::new(String::from("multi thread"));
    // for _ in 0..10 {
    //     let s = Rc::clone(&s);
    //     let handle = thread::spawn(move || {
    //        println!("{}", s)
    //     });
    // }

    let s = Arc::new(String::from("multi thread"));
    for i in 0..10 {
        let s = Arc::clone(&s);
        let handle = thread::spawn(move || {
           println!("{} {}", s, i)
        });
    }
}
