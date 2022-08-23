use std::cell::Cell;

fn main() {
    // let c = Cell::new("asdf");
    let c = Cell::new(String::from("asdf"));
    let one = c.get();
    c.set("qwer".to_string());
    let two = c.get();
    println!("{},{}", one, two);
}
