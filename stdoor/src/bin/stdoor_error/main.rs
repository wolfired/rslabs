use std::error::Error;
use std::fmt::Display;

/// std::error
/// 本模块定义了错误相关的特性
/// 
/// std::error::Error
/// 错误特性

#[derive(Debug)]
struct LowerError;

impl Display for LowerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LowerError is here!")
    }
}

impl Error for LowerError {}

#[derive(Debug)]
struct HigherError(LowerError);

impl Display for HigherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HigherError is here!")
    }
}

impl Error for HigherError{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.0)
    }
}

fn main() {
    let le = LowerError;
    println!("{}", le);

    let he = HigherError(le);
    println!("{}", he);
    println!("{}", he.source().unwrap());
}