#![allow(dead_code)]

use stdoor::trace;

fn main() {
    smallest_largest_value();
    
    all_abs();
}

fn smallest_largest_value() {
    println!("\n定义在各模块的有符号整型最小最大值(已被取代)");
    trace("i8 smallest value", 32, format!("std::i8::MIN = {}", std::i8::MIN).as_str(), None);
    trace("i8 largest value", 32, format!("std::i8::MAX = {}", std::i8::MAX).as_str(), None);
    trace("i16 smallest value", 32, format!("std::i16::MIN = {}", std::i16::MIN).as_str(), None);
    trace("i16 largest value", 32, format!("std::i16::MAX = {}", std::i16::MAX).as_str(), None);
    trace("i32 smallest value", 32, format!("std::i32::MIN = {}", std::i32::MIN).as_str(), None);
    trace("i32 largest value", 32, format!("std::i32::MAX = {}", std::i32::MAX).as_str(), None);
    trace("i64 smallest value", 32, format!("std::i64::MIN = {}", std::i64::MIN).as_str(), None);
    trace("i64 largest value", 32, format!("std::i64::MAX = {}", std::i64::MAX).as_str(), None);
    trace("i128 smallest value", 32, format!("std::i128::MIN = {}", std::i128::MIN).as_str(), None);
    trace("i128 largest value", 32, format!("std::i128::MAX = {}", std::i128::MAX).as_str(), None);
    trace("isize smallest value", 32, format!("std::isize::MIN = {}", std::isize::MIN).as_str(), None);
    trace("isize largest value", 32, format!("std::isize::MAX = {}", std::isize::MAX).as_str(), None);

    println!("\n定义在各原始类型的有符号整型最小最大值");
    trace("i8 smallest value", 32, format!("i8::MIN = {}", i8::MIN).as_str(), None);
    trace("i8 largest value", 32, format!("i8::MAX = {}", i8::MAX).as_str(), None);
    trace("i16 smallest value", 32, format!("i16::MIN = {}", i16::MIN).as_str(), None);
    trace("i16 largest value", 32, format!("i16::MAX = {}", i16::MAX).as_str(), None);
    trace("i32 smallest value", 32, format!("i32::MIN = {}", i32::MIN).as_str(), None);
    trace("i32 largest value", 32, format!("i32::MAX = {}", i32::MAX).as_str(), None);
    trace("i64 smallest value", 32, format!("i64::MIN = {}", i64::MIN).as_str(), None);
    trace("i64 largest value", 32, format!("i64::MAX = {}", i64::MAX).as_str(), None);
    trace("i128 smallest value", 32, format!("i128::MIN = {}", i128::MIN).as_str(), None);
    trace("i128 largest value", 32, format!("i128::MAX = {}", i128::MAX).as_str(), None);
    trace("isize smallest value", 32, format!("isize::MIN = {}", isize::MIN).as_str(), None);
    trace("isize largest value", 32, format!("isize::MAX = {}", isize::MAX).as_str(), None);
    
    println!("\n定义在各模块的无符号整型最小最大值(已被取代)");
    trace("u8 smallest value", 32, format!("std::u8::MIN = {}", std::u8::MIN).as_str(), None);
    trace("u8 largest value", 32, format!("std::u8::MAX = {}", std::u8::MAX).as_str(), None);
    trace("u16 smallest value", 32, format!("std::u16::MIN = {}", std::u16::MIN).as_str(), None);
    trace("u16 largest value", 32, format!("std::u16::MAX = {}", std::u16::MAX).as_str(), None);
    trace("u32 smallest value", 32, format!("std::u32::MIN = {}", std::u32::MIN).as_str(), None);
    trace("u32 largest value", 32, format!("std::u32::MAX = {}", std::u32::MAX).as_str(), None);
    trace("u64 smallest value", 32, format!("std::u64::MIN = {}", std::u64::MIN).as_str(), None);
    trace("u64 largest value", 32, format!("std::u64::MAX = {}", std::u64::MAX).as_str(), None);
    trace("u128 smallest value", 32, format!("std::u128::MIN = {}", std::u128::MIN).as_str(), None);
    trace("u128 largest value", 32, format!("std::u128::MAX = {}", std::u128::MAX).as_str(), None);
    trace("usize smallest value", 32, format!("std::usize::MIN = {}", std::usize::MIN).as_str(), None);
    trace("usize largest value", 32, format!("std::usize::MAX = {}", std::usize::MAX).as_str(), None);

    println!("\n定义在各原始类型的无符号整型最小最大值");
    trace("u8 smallest value", 32, format!("u8::MIN = {}", u8::MIN).as_str(), None);
    trace("u8 largest value", 32, format!("u8::MAX = {}", u8::MAX).as_str(), None);
    trace("u16 smallest value", 32, format!("u16::MIN = {}", u16::MIN).as_str(), None);
    trace("u16 largest value", 32, format!("u16::MAX = {}", u16::MAX).as_str(), None);
    trace("u32 smallest value", 32, format!("u32::MIN = {}", u32::MIN).as_str(), None);
    trace("u32 largest value", 32, format!("u32::MAX = {}", u32::MAX).as_str(), None);
    trace("u64 smallest value", 32, format!("u64::MIN = {}", u64::MIN).as_str(), None);
    trace("u64 largest value", 32, format!("u64::MAX = {}", u64::MAX).as_str(), None);
    trace("u128 smallest value", 32, format!("u128::MIN = {}", u128::MIN).as_str(), None);
    trace("u128 largest value", 32, format!("u128::MAX = {}", u128::MAX).as_str(), None);
    trace("usize smallest value", 32, format!("usize::MIN = {}", usize::MIN).as_str(), None);
    trace("usize largest value", 32, format!("usize::MAX = {}", usize::MAX).as_str(), None);
}

fn all_abs() {
    {
        // abs取绝对值(不安全)
        let x = -1i8;
        trace(format!("abs({}) = ", x).as_str(), 32, format!("{:<}", x.abs()).as_str(), Some(""));

        if cfg!(debug_assertions) {
            trace(format!("abs({}) = ", i8::MIN).as_str(), 32, format!("a panic in debug mode").as_str(), Some(""));
        } else {
            //在 Debug 模式下会报错, 在 Release 模式下不会报错且返回 i8::MIN
            trace(format!("abs({}) = ", i8::MIN).as_str(), 32, format!("{:<}", i8::MIN.abs()).as_str(), Some(""));
        }
    }

    {
        // checked_abs取绝对值
        let x = -1i8;
        trace(format!("checked_abs({}) = ", x).as_str(), 32, format!("{:<}", x.checked_abs().unwrap()).as_str(), Some(""));

        if let None = i8::MIN.checked_abs() {
            trace(format!("checked_abs({}) = ", i8::MIN).as_str(), 32, format!("None").as_str(), Some(""));
        } 
    }

    {
        // overflowing_abs取绝对值, 返回元组(i8, bool), t.0=正确的绝对值 或 i8::MIN, t.1=是否溢出
        let x = -1i8;
        trace(format!("overflowing_abs({}) = ", x).as_str(), 32, format!("{:<?}", x.overflowing_abs()).as_str(), Some(""));

        trace(format!("overflowing_abs({}) = ", i8::MIN).as_str(), 32, format!("{:<?}", i8::MIN.overflowing_abs()).as_str(), Some(""));
    }

    {
        // saturating_abs取绝对值, 如果溢出, 返回 i8::MAX
        let x = -1i8;
        trace(format!("saturating_abs({}) = ", x).as_str(), 32, format!("{:<}", x.saturating_abs()).as_str(), Some(""));

        trace(format!("saturating_abs({}) = ", i8::MIN).as_str(), 32, format!("{:<}", i8::MIN.saturating_abs()).as_str(), Some(""));
    }

    {
        // wrapping_abs取绝对值, 如果溢出, 返回 i8::MIN
        let x = -1i8;
        trace(format!("wrapping_abs({}) = ", x).as_str(), 32, format!("{:<}", x.wrapping_abs()).as_str(), Some(""));

        trace(format!("wrapping_abs({}) = ", i8::MIN).as_str(), 32, format!("{:<}", i8::MIN.wrapping_abs()).as_str(), Some(""));
    }
}

fn all_add() {
    
}
