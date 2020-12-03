//! std::option<br/>
//! "可选"模块, 这个模块最重要的就是`Option`这个枚举类型<br/>
//! `Option`代表一个可选值: 它可能是包含一个具体值的`Some`, 也可能是"空值"`None`<br/>
//! `Option`有以下用途:<br/>
//! 1. 初始化值<br/>
//! 2. 函数输入在定义域外返回`None`<br/>
//! 3. 函数发生错误返回`None`<br/>
//! 4. 可选的结构体字段<br/>
//! 5. 可以"借入"或"借出"的结构体字段<br/>
//! 6. 可选的函数参数<br/>
//! 7. 可空的指针<br/>
//! 8. Swapping things out of difficult situations<br/>

#![allow(non_snake_case)]
#![allow(dead_code)]

fn main() {
    work_with_match();
    nullable_pointer();
}

/// `Option`通常与`match`一起使用, 判断值是否存在再进行处理
fn work_with_match() {
    let value: Option<u8> = None;
    // let value:Option<u8> = Some(0);

    match value {
        Some(_) => println!("value is not none"),
        None => println!("value is none"),
    }

    if let Some(_) = value {
        println!("value is not none")
    } else {
        println!("value is none")
    }
}

/// 空指针
fn nullable_pointer(){
    // let ptr:Option<Box<i32>> = None;
    let ptr:Option<Box<i32>> = Some(Box::new(0));

    match ptr {
        Some(_) => println!("ptr is not none"),
        None => println!("ptr is none"),
    }
}
