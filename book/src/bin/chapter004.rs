#![allow(unused_variables)]
/**
 * 理解所有权(Understanding Ownership)
 */
fn main() {
    section001();
    section002();
    section003();
}

#[derive(Debug)]
struct P {
    age: i32,
}

/**
 * 什么是所有权?(What Is Ownership?)
 */
fn section001() {
    {
        // 所有权规则:
        // 1. Rust中的每个值(变量)都有一个被称为所有者的变量
        // 2. 每个值(变量)在一个时间点只能有一个所有者
        // 3. 当所有者离开作用域, 值(变量)将会被删除
        let v0 = "v0";
        {
            let v1 = "v1";
        }
        println!("{0}", v0);
        // println!("{0}", v1); // error code: 超出了_s1的作用域
    }

    {
        // 变量与数据交互的方式: 移动(Ways Variables and Data Interact: Move)
        // 一个数据类型的一个实际值所占用的内存, 只有两种情况, 要么全在栈上, 要么部分在栈上部分在堆里
        // 对一个值进行复制, 仅复制栈上的那部分被称为浅复制, 同时把堆里那部分也复制了被称为深复制
        // 对于一个全在栈上的值, 浅复制与深复制是一样的, 复制后是完全独立的两个值
        // 对于一个部分在栈上部分在堆里的值, 浅复制后，栈上那部分值是完全独立的而堆里那部分数据共享, 深复制才是完全独立的两个值
        // Rust把源值标记为无效而不进行浅复制, 这被称为移动
        // 移动发生在三个地方: 赋值, 函数/方法传参, 函数/方法返回值
        let v0 = String::from("v0");
        let v1 = v0; // v0被移动, 不再有效

        let v0 = [0];
        let v1 = v0;
        println!("{:?}, {:?}", v0, v1);

        let v0 = [P { age: 0 }];
        let v1 = v0;
        println!("{:?}", v1); // v0被移动, 不再有效
    }

    {
        // 变量与数据交互的方式: 克隆(Ways Variables and Data Interact: Clone)
        let v0 = String::from("v0");
        let v1 = v0.clone(); // v0依然有效

        let v0 = P { age: 0 };
    }

    {
        // 仅存在于栈的数据: 复制(Stack-Only Data: Copy)
        // Rust有一个叫Copy的特征(trait), 所有实现这个特征的类型在其进行值复制时都是深复制, Copy特征与Drop特征互斥
        // 标量类型都是Copy的, 包括: 整型, 浮点型, 布尔型, 字符型
        // 自定义数据类型: 枚举和结构体, 不是Copy的
        // 复合数据类型: 元组和数组, 只要元素都是Copy的, 那就是Copy的
        // PS: 在我的理解里, 引用也是Copy的
        let x = 5;
        let y = x;
        println!("{0}, {1}", x, y);

        let x = (0, 0.0);
        let y = x;
        println!("{:?}, {:?}", x, y);

        let x = [0; 1];
        let y = x;
        println!("{:?}, {:?}", x, y);

        let x = P { age: 0 };
        let y = &x;
        let z = y;
        println!("{:?}, {:?}, {:?}", x, y, z);
    }

    {
        // 所有权与函数(Ownership and Functions)
        let v0 = P { age: 0 };
        let v1 = move_ownership(v0);
        println!("{:?}", v1);
    }
}

fn move_ownership(p: P) -> P {
    return p;
}

/**
 * 引用(名词)与借用(动词)(References and Borrowing)
 *
 * 引用是一种数据类型, 它指向一个值而不获取该值的所有权
 *
 * 使用引用的编程方式被称为借用
 *
 * 引用规则:
 *
 * 1. 在任意时间点, 在同一作用域内, 要么只有一个可写引用, 要么只有多个只读引用
 *
 * 2. 引用必须有效
 */
fn section002() {
    let v = P { age: 0 };
    let vr = &v; // vr是对v的引用
    unmove_ownership(&v); // 借用v, 本质上就是获得一个对v的引用
    unmove_ownership(vr);
    println!("{:?}, {:?}", v, vr);
}

fn unmove_ownership(p: &P) {}

/**
 * 切片(Slice)类型(The Slice Type)
 *
 * 切片是一种没有所有权的数据类型
 *
 * 切片指向一个集合的一个子集, 这个子集的元素是相邻的
 */
fn section003() {
    let v0 = "abc";
    let v1 = &v0[..];
}
