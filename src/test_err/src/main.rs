use std::fmt::{Debug, Display, Formatter};
use std::fs::{File, read_to_string};
use std::io;
use std::io::Error;
use std::num::ParseIntError;

struct AppError {
    code: usize,
    message: String,
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let err_msg = match self.code {
            404 => "Sorry, Can not find the Page!",
            _ => "Sorry, something is wrong! Please Try Again!",
        };

        write!(f, "{}", err_msg)
    }
}

impl Debug for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "AppError {{ code: {}, message: {} }}", self.code, self.message)
    }
}

fn produce_error() -> Result<(), AppError> {
    Err(AppError {
        code: 404,
        message: String::from("Page not found"),
    })
}

#[derive(Debug)]
struct AppError2 {
    kind: String,
    message: String,
}

impl From<io::Error> for AppError2 {
    fn from(error: Error) -> Self {
        AppError2 {
            kind: String::from("io"),
            message: error.to_string(),
        }
    }
}

impl From<ParseIntError> for AppError2 {
    fn from(error: ParseIntError) -> Self {
        AppError2 {
            kind: String::from("parse"),
            message: error.to_string(),
        }
    }
}

// fn main() -> Result<(), AppError2> {
//     // let _file = File::open("nonexist.txt")?;
//
//     let parse_value: usize = "hello".parse()?;
//     Ok(())
// }

// fn main() {
//     let s1 = Some("some1");
//     let s2 = Some("some2");
//     let n: Option<&str> = None;
//
//     let o1: Result<&str, &str> = Ok("ok1");
//     let o2: Result<&str, &str> = Ok("ok2");
//     let e1: Result<&str, &str> = Err("error1");
//     let e2: Result<&str, &str> = Err("error2");
//
//     //s1
//     assert_eq!(s1.or(s2), s1);
//     //s1
//     assert_eq!(s1.or(n), s1);
//     //s1
//     assert_eq!(n.or(s1), s1);
//     //n
//     assert_eq!(n.or(n), n);
//
//     //o1
//     assert_eq!(o1.or(o2), o1);
//     //o1
//     assert_eq!(o1.or(e1), o1);
//
//     //o1
//     assert_eq!(e1.or(o1), o1);
//
//     //e2
//     assert_eq!(e1.or(e2), e2);
//
//
//     //s2
//     assert_eq!(s1.and(s2), s2);
//
//     //n
//     assert_eq!(s1.and(n), n);
//
//     //n
//     assert_eq!(n.and(s1), n);
//
//     //n
//     assert_eq!(n.and(n), n);
//
//
//     //o2
//     assert_eq!(o1.and(o2), o2);
//
//     //e1
//     assert_eq!(o1.and(e1), e1);
//
//     //e1
//     assert_eq!(e1.and(o1), e1);
//
//     //e1
//     assert_eq!(e1.and(e2), e1);
//
//     let fn_name = || Some("some2");
//     let fn_none = || None;
//
//     assert_eq!(s1.or_else(fn_name), s1);
//
//     assert_eq!(s1.or_else(fn_none), s1);
//
//     let s = Some(10);
//     let n: Option<i8> = None;
//
//     let fn_closure = |v: i8| v + 2;
//     let fn_default = || 1;
//
//     assert_eq!(s.map_or_else(fn_default, fn_closure), 12);
//     assert_eq!(n.map_or_else(fn_default, fn_closure), 1);
//
//     const ERR_DEFAULT: &str = "error message";
//     let s = Some("abcde");
//     let n: Option<&str> = None;
//
//     let o: Result<&str, &str> = Ok("abcde");
//     let e: Result<&str, &str> = Err(ERR_DEFAULT);
//
//     assert_eq!(s.ok_or(ERR_DEFAULT), o);
//     assert_eq!(n.ok_or(ERR_DEFAULT), e);
//
//     assert_eq!(s.ok_or_else(|| "error message"), o);
//     assert_eq!(n.ok_or_else(|| "error message"), e);
//
//     match produce_error() {
//         Err(e) => {
//             eprintln!("{}", e)
//         }
//         _ => println!("No error")
//     }
//     eprintln!("{:?}", produce_error());
//     eprintln!("{:#?}", produce_error());
// }

// fn main()->Result<(), Box<dyn std::error::Error>>{
//     let html = render()?;
//     println!("{}", html);
//     Ok(())
// }
//
// fn render() -> Result<String, Box<dyn std::error::Error>>{
//     let file = std::env::var("MARKDOWN")?;
//     let source = read_to_string(file)?;
//     Ok(source)
// }

fn main()->Result<(), MyError>{
    let html = render()?;
    println!("{}", html);
    Ok(())
}

fn render() -> Result<String, MyError>{
    let file = std::env::var("MARKDOWN")?;
    let source = read_to_string(file)?;
    Ok(source)
}

#[derive(thiserror::Error, Debug)]
enum MyError {
    #[error("Environment variable not found")]
    EnvironmentVariableNotFound(#[from] std::env::VarError),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}