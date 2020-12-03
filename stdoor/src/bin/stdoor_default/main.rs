//! std::default
//! 默认模块, 用于获得一个类型的有意义默认实例(值)
//! 此模块包含一个派生宏, 一个特征, 一个函数

#![allow(dead_code)]
#![allow(unused_imports)]

use stdoor::trace;

#[derive(Default, Debug)]
struct A {
    age: i8,
}

#[derive(Debug)]
struct B {
    age: i8,
}

impl Default for B {
    fn default() -> Self {
        B { age: 1 }
    }
}

fn main() {
    let a: A = Default::default();
    let b = B::default();

    println!("{:#?}", a);
    println!("{:#?}", b);
}
