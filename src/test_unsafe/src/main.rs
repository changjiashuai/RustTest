use std::slice;
use std::slice::from_raw_parts;
use std::str::from_utf8_unchecked;

fn get_memory_location() -> (usize, usize) {
    let string = "Hello World!";
    let pointer = string.as_ptr() as usize;
    let length = string.len();
    (pointer, length)
}

fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
    unsafe {
        from_utf8_unchecked(from_raw_parts(pointer as *const u8, length))
    }
}

unsafe fn dangerous() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // unsafe {
    println!("r1 is: {}", *r1);
    *r2 = 3;
    println!("r2 is: {}", *r2);
    // }
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    // (&mut slice[..mid], &mut slice[mid..])

    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
         slice::from_raw_parts_mut(ptr.add(mid), len - mid))
    }
}

extern "C" {
  fn abs(input: i32) -> i32;
}

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        *r2 = 3;
        println!("r2 is: {}", *r2);
    }

    //danger
    let address = 0x012345usize;
    let r = address as *const i32;
    // unsafe { println!("r is: {}", *r); }

    let (pointer, length) = get_memory_location();
    let message = get_str_at_location(pointer, length);
    println!("The {} bytes at 0x{:X} stored: {}", length, pointer, message);

    // let message = get_str_at_location(1000, 10);
    // println!("msg={}", message)

    let a: Box<i32> = Box::new(10);
    let b: *const i32 = &*a as *const i32;
    let c: *const i32 = Box::into_raw(a);

    unsafe { dangerous() }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = split_at_mut(r, 3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

#[repr(C)]
union MyUnion {
    f1:u32,
    f2:f32,
}
