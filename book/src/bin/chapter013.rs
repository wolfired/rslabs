/**
 * 函数式语言特性: 迭代器与闭包(Functional Language Features: Iterators and Closures)
 */
fn main() {
    section001();
    section002();
}

#[derive(Debug)]
struct P {
    age: i8,
}

/**
 * 闭包: 可以捕获它们所处环境的匿名函数(Closures: Anonymous Functions that Can Capture Their Environment)
 */
fn section001() {
    let p = &P { age: 0 };
    let func = || {
        println!("Hi, {0}", p.age);
    };
    println!("{:?}", p);
    func();
}

/**
 * 使用迭代器处理连续项(Processing a Series of Items with Iterators)
 */
fn section002() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}
