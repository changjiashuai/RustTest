use test_proc_macro_derive::HelloMacro;
use hello_macro::HelloMacro;

#[derive(HelloMacro)]
struct Hello;

struct World;

impl HelloMacro for World {
    fn hello_macro() {
        println!("World impl")
    }
}

fn main() {
    println!("Hello, world!");
    println!{"Hello"};
    println!["hello"];

    let v = vec2!["hello"];
    let v = vec2!{
        "a", "b", "c"
    };
    println!("v={:?}", v);

    let a = Hello::hello_macro();
    let b = World::hello_macro();
}

#[macro_export]
macro_rules! vec2 {
    ($( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}