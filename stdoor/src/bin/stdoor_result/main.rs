//! `std::result`
//! 
//! 使用`Result`进行错误处理, `Result<T, E>`用于返回和传播错误, 这是个枚举
//! 
//! `OK(T)`表示成功并保存值
//! 
//! `Err(E)`表示错误并保存错误值

use stdoor::trace;

fn main() {
    match error_inner(63) {
        Ok(v) => trace("Ok", 32, format!("v = {}, v < 64", v).as_str(), None),
        Err(v) => trace("Err", 32, format!("v = {}, v >= 64", v).as_str(), None),
    }

    match error_outer(64) {
        Ok(v) => trace("Ok", 32, format!("v = {}, v < 64", v).as_str(), None),
        Err(v) => trace("Err", 32, format!("v = {}, v >= 64", v).as_str(), None),
    }

    test_and();
    test_or();
    test_ok_err();
    test_expect();
    test_map();
}

type Rii = Result<i8, i8>;

fn error_inner(v: i8) -> Rii {
    if 64 > v {
        Ok(v)
    } else {
        Err(v)
    }
}

/// "?" 操作符
/// 
/// 如果一个函数返回`Result`, 而其内部调用别一个函数也返回同一`Result`, 则可以使用`?` 简化代码
fn error_outer(v: i8) -> Rii {
    let v = error_inner(v)?;
    Ok(v)
}

fn test_and() {
    {
        // 如果 x 是 Ok, 返回 y
        // 如果 x 是 Err, 返回 x
        let x: Result<i8, i8> = Err(0);
        let y: Result<u8, i8> = Err(1);
        let z = x.and(y);
        trace(
            format!("x = {:?}, y = {:?}, x.and(y) = ", x, y).as_str(),
            42,
            format!("{:?}", z).as_str(),
            Some(""),
        );
    }

    {
        // 如果 x 是 Ok, 执行 double
        // 如果 x 是 Err, 返回 x
        let x: Result<i8, i8> = Ok(2);
        let double = |x: i8| -> Result<u8, i8> { Ok((x as u8) * (x as u8)) };
        let z = x.and_then(double);
        trace(
            format!("x = {:?} x.and_then(double) = ", x).as_str(),
            42,
            format!("{:?}", z).as_str(),
            Some(""),
        );
    }
}

fn test_or() {
    {
        // 如果 x 是 Err, 返回 y
        // 如果 x 是 Ok, 返回 x
        let x: Result<i8, i8> = Err(0);
        let y: Result<i8, u8> = Err(1);
        let z = x.or(y);
        trace(
            format!("x = {:?}, y = {:?}, x.or(y) = ", x, y).as_str(),
            42,
            format!("{:?}", z).as_str(),
            Some(""),
        );
    }

    {
        // 如果 x 是 Err, 执行 double
        // 如果 x 是 Ok, 返回 x
        let x: Result<i8, i8> = Err(2);
        let double = |x: i8| -> Result<i8, u8> { Ok(x * x) };
        let z = x.or_else(double);
        trace(
            format!("x = {:?} x.or_else(double) = ", x).as_str(),
            42,
            format!("{:?}", z).as_str(),
            Some(""),
        );
    }
}

fn test_ok_err() {
    {
        let x: Result<i8, i8> = Err(0);
        if let Some(x) = x.ok() {
            trace("x is Ok: ", 32, format!("{}", x).as_str(), Some(""));
        } else {
            trace("x is Err", 32, "", None);
        }
    }

    {
        let x: Result<i8, i8> = Err(0);
        if let Some(x) = x.err() {
            trace("x is Err: ", 32, format!("{}", x).as_str(), Some(""));
        } else {
            trace("x is Ok", 32, "", None);
        }
    }
}

fn test_expect() {
    {
        // 消耗自身, 返回 Ok 包含的值
        // 如果是 Err, 报错, 输出传的字符串以及 Err 包含的值
        let x: Result<i8, i8> = Ok(0);
        // let x: Result<i8, i8> = Err(1);
        trace(
            "x is Ok: ",
            32,
            format!("{}", x.expect("x is Err, and value is")).as_str(),
            Some(""),
        );
    }

    {
        // 消耗自身, 返回 Err 包含的值
        // 如果是 Ok, 报错, 输出传的字符串以及 Ok 包含的值
        // let x: Result<i8, i8> = Ok(0);
        let x: Result<i8, i8> = Err(1);
        trace(
            "x is Err: ",
            32,
            format!("{}", x.expect_err("x is Ok, and value is")).as_str(),
            Some(""),
        );
    }
}

fn test_map() {
    {
        let x: Result<i8, i8> = Ok(2);
        let double = |x| (x * x) as u8;
        if let Ok(v) = x.map(double) {
            trace(format!("x is {:?}, double it: ", x).as_str(), 32, format!("{}", v).as_str(), Some(""));
        } else {
            trace("x is Err", 32, "", Some(""));
        }
    }
}
