#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_parens)]
#![allow(dead_code)]

/**
 * 通用编程概念(Common Programming Concepts)
 *
 * 本章涵盖了几乎在所有编程语言中都出现的概念, 并展示了它们在Rust中如何工作. 
 * 很多编程语言的核心有着诸多共同点
 * 本章介绍的概念都不是Rust独有的, 我们将在Rust的背景下讨论它们, 解释围绕使用这些概念的约定.
 * 
 * 具体来说, 你将学习到变量, 基础类型, 函数, 注释和控制流. 这些基础将出现在每个Rust程序当中,
 * 及早学习它们将为您的后续学习打下坚实的基础.
 * 
 * 关键字:
 * 与其它语言一样, Rust有一组仅允许被语言使用的保留关键字. 你要记住不能把这些关键字用作变量名或函数名.
 * 多数关键字有着特殊的涵义, 你可以在你的Rust程序中使用它们来完成各种任务.
 * 少数关键字目前没有与之对应的功能, 但依然被保留起来, 以便将来Rust增加新功能时使用.
 * 你可以在附录A中查看全部关键字
 */
fn main() {
    section001();
    section002();
    section003();
    section005();
}

const GLOBAL_CONSTANT: i32 = 0; // 可以定义全局常量, 但不能定义全局变量

/**
 * 变量与易变性(Variables and Mutability)
 */
fn section001() {
    {
        let x: i32; // 使用let声明变量, 其值在初始化后只读
        x = 0; // 初始化
               // x = 1; // error

        let y = 0; // 使用let声明并初始化变量, 变量类型可以忽略由具体初始值决定, 其值只读
                   // y = 1; // error

        let mut z = 0; // 使用let mut声明并初始化变量, 其值可写
        z = 1;
    }

    {
        const LOCAL_CONSTANT: i32 = GLOBAL_CONSTANT; // 使用const声明常量, 必须同时提供常量数据类型, 必须在声明的同时初始化常量值
                                                     // 只接受常量表达式作为右值, 也就是说函数调用返回值或其它运行时运算值均非法
    }

    {
        // 变量隐藏(Shadow), 常量不能隐藏
        let x = 0;
        let x = 0.0;
        let x = 0;
        let mut x = 0.0;
        x = 1.0;
        let mut x = 0;
        x = 1;
        let x = 0.0;
    }
}

/**
 * 数据类型(Data Types)
 *
 * 每个值都有一个具体的数据类型
 *
 * 本节我们先来看看其中两类数据类型: 标量(scalar)类型 与 复合(compound)类型
 */
fn section002() {
    {
        // Rust有四种标量类型: 整型, 浮点型, 布尔型, 字符型

        {
            // 整型
            let v = 0i8; // 8位有符号整型
            let v = 0u8; // 8位无符号整型

            let v = 0i16; // 16位有符号整型
            let v = 0u16; // 16位无符号整型

            let v = 0; // 32位有符号整型(默认)
            let v = 0u32; // 32位无符号整型

            let v = 0i64; // 64位有符号整型
            let v = 0u64; // 64位无符号整型

            let v = 0isize; // 根据系统架构定
            let v = 0usize; // 根据系统架构定

            let v = 1_024; // 十进制(默认), _作为虚拟分隔符方便阅读
            let v = 0xFFFF_FFFFu32; // 十六进制
            let v = 0o77; // 八进制
            let v = 0b1111_0000; // 二进制
            let v = b'F'; // 字节, 上面4种写法都可以在值后写上类型以指定变量类型, 而这种写法限定是u8
        }

        {
            // 浮点型
            let v: f32 = 0.0; // 32位浮点数
            let v: f64 = 0.0; // 64位浮点数(默认)
        }

        {
            // 布尔型
            let v: bool = true;
            let v = false;
        }

        {
            // 字符型, 使用单引号
            let v: char = '\0';
        }
    }

    {
        // Rust有两种复合类型: 元组(tuple), 数组(array)
        {
            // 元组, 元素数据类型和元素数量都是在编译期就能确定的
            // 类型声明: (type0, type1, ...)

            let v = (); // 空元组
                        // v = (); // error: v只读, 不能修改v的值

            let v = (0, 0.0);
            // v.0 = 1; // error: v只读, 不能修改v内部的值

            let mut v = (0, 0.0);
            v = (1, 1.0);
            v.0 = 2; // 通过.索引操作符读写元组的元素

            let mut v0 = (0, 0.0, true);
            let v1 = (1, 1.0, false);
            v0 = v1; // 元组类型元素一一对应即认为两个元组类型相同

            let v = (0, 0.0);
            let (x, mut y) = v; // 解构赋值
                                // x = 1; // error: x只读
            y = 1.0;

            let mut v = (0, 0.0);
            let x = &mut v.0;
            *x = 1; // v的值变成[1, 0.0]

            let v = (0); // 当元组只有一个元素时, 元组将退化成唯一元素对应的数据类型
        }

        {
            // 数组, 素数据类型和元素数量都是在编译期就能确定的
            // 类型声明: [type; length]
            // 值初始化: [value0, value1, ...] || [initvalue; length]

            let v = [0; 0]; //空数组
                            // v = [0; 0]; // error: v只读, 不能修改v的值

            let v = [0; 1];
            // v[0] = 1; // error: v只读, 不能修改v内部的值

            let mut v = [0; 1];
            v = [1; 1];
            v[0] = 2; // 通过索引操作数组的元素

            let v = [0, 0]; // 声明并初始化一个长度为1的i32数组, 唯一的元素值为1
            let [a, mut b] = v; // 解构赋值
                                // a = 1; // error: a只读
            b = 1;

            let mut v = [0, 0];
            let a = &mut v[0];
            *a = 1; // v的值变成[1, 0]
        }
    }
}

/**
 * 函数(Functions)
 *
 * 使用 fn 定义一个函数, -> 指定返回值类型
 */
fn section003() -> i32 {
    let v = (0, 1);
    println!("{:?}", swap_a_tuple(v));

    let v = [0, 1];
    println!("{:?}", swap_a_array(v));

    return 0;
}

/// 交换元组的两个元素并返回
fn swap_a_tuple((a, b): (i32, i32)) -> (i32, i32) {
    return (b, a); // 语句
}

/// 交换数组的两个元素并返回
fn swap_a_array([a, b]: [i32; 2]) -> [i32; 2] {
    [b, a] // 表达式
}

/**
 * 控制流(Control Flow)
 */
fn section005() {
    {
        // if表达式, 只接受布尔型
        let v = true;
        // let v = 0; // error
        if v {
            println!("condition was true");
        } else {
            println!("condition was false");
        }

        let v = 0;
        if 0 == v {
            println!("v is 0");
        } else if 1 == v {
            println!("v is 1");
        } else {
            println!("v is non 0 and non 1");
        }

        let c = true;
        let v = if c {
            0
        } else {
            // 如果不写else语句会报错
            // 如果分支的数据类型不相同也会报错
            1
        }; // 这个分号不能漏
    }

    {
        // Rust有三种循环: loop, while, for
        {
            // loop
            loop {
                println!("if i don't break, it will loop forever!!");
                break;
            }

            let mut c = 0;
            let r = loop {
                c += 1;

                if 2 == c {
                    break c * 2; // 仅对于loop循环, break语句会返回其后面的值, 如果省略值, 则返回空元组
                }
            }; // 这个分号不能漏
            println!("r is {0}", r);
        }

        {
            // while
            let mut c = 2;
            while 0 != c {
                println!("{}!", c);
                c -= 1;
            }
        }

        {
            // for, 迭代数组, 集合, 映射的所有元素
            let arr = [0, 1, 2, 3];
            for a in arr.iter() {
                println!("{}!", a);
            }
        }
    }
}
