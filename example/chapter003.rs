/**
 * 常见的编程概念(Common Programming Concepts)
 */
fn main() {
    section001();
    section002();
    section003();
    section005();
}

const _GLOBAL_CONSTANT: i32 = 0; // 可以定义全局常量, 但不能定义全局变量

/**
 * 变量与易变性(Variables and Mutability)
 */
fn section001() {
    {
        let _x: i32; // 使用let声明变量, 其值在初始化后只读
        _x = 0; // 初始化
                // _x = 1; // error

        let _y = 0; // 使用let声明并初始化变量, 变量类型可以忽略由具体初始值决定, 其值只读
                    // _y = 1; // error

        let mut _z = 0; // 使用let mut声明并初始化变量, 其值可写
        _z = 1;
    }

    {
        const _LOCAL_CONSTANT: i32 = _GLOBAL_CONSTANT; // 使用const声明常量, 必须同时提供常量数据类型, 必须在声明的同时初始化常量值
                                                       // 只接受常量表达式作为右值, 也就是说函数调用返回值或其它运行时运算值均非法
    }

    {
        // 变量隐藏(Shadow), 常量不能隐藏
        let _x = 0;
        let _x = 0.0;
        let _x = 0;
        let mut _x = 0.0;
        _x = 1.0;
        let mut _x = 0;
        _x = 1;
        let _x = 0.0;
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
            let _v = 0i8; // 8位有符号整型
            let _v = 0u8; // 8位无符号整型

            let _v = 0i16; // 16位有符号整型
            let _v = 0u16; // 16位无符号整型

            let _v = 0; // 32位有符号整型(默认)
            let _v = 0u32; // 32位无符号整型

            let _v = 0i64; // 64位有符号整型
            let _v = 0u64; // 64位无符号整型

            let _v = 0isize; // 根据系统架构定
            let _v = 0usize; // 根据系统架构定

            let _v = 1_024; // 十进制(默认), _作为虚拟分隔符方便阅读
            let _v = 0xFFFF_FFFFu32; // 十六进制
            let _v = 0o77; // 八进制
            let _v = 0b1111_0000; // 二进制
            let _v = b'F'; // 字节, 上面4种写法都可以在值后写上类型以指定变量类型, 而这种写法限定是u8
        }

        {
            // 浮点型
            let _v: f32 = 0.0; // 32位浮点数
            let _v: f64 = 0.0; // 64位浮点数(默认)
        }

        {
            // 布尔型
            let _v: bool = true;
            let _v = false;
        }

        {
            // 字符型, 使用单引号
            let _v: char = '\0';
        }
    }

    {
        // Rust有两种复合类型: 元组(tuple), 数组(array)
        {
            // 元组, 元素数据类型和元素数量都是在编译期就能确定的
            // 类型声明: (type0, type1, ...)

            let _v = (); // 空元组
                         // _v = (); // error: _v只读, 不能修改_v的值
            let _v = (0, 0.0);
            // _v.0 = 1; // error: _v只读, 不能修改_v内部的值
            let mut _v = (0, 0.0);
            _v = (1, 1.0);
            _v.0 = 2; // 通过.索引操作符读写元组的元素

            let mut _v0 = (0, 0.0, true);
            let _v1 = (1, 1.0, false);
            _v0 = _v1; // 元组类型元素一一对应即认为两个元组类型相同

            let _v = (0, 0.0);
            let (_x, mut _y) = _v; // 解构赋值
                                   // _x = 1; // error: _x只读
            _y = 1.0;

            let mut _v = (0, 0.0);
            let _x = &mut _v.0;
            *_x = 1; // _v的值变成[1, 0.0]

            let _v = (0); // 当元组只有一个元素时, 元组将退化成元素对应的数据类型
        }

        {
            // 数组, 素数据类型和元素数量都是在编译期就能确定的
            // 类型声明: [type; length]
            // 值初始化: [value0, value1, ...] || [init_value; length]

            let _v = [0; 0]; //空数组
                             // _v = [0; 0]; // error: _v只读, 不能修改_v的值
            let _v = [0; 1];
            // _v[0] = 1; // error: _v只读, 不能修改_v内部的值
            let mut _v = [0; 1];
            _v = [1; 1];
            _v[0] = 2; // 通过索引操作数组的元素

            let _v = [0, 0]; // 声明并初始化一个长度为1的i32数组, 唯一的元素值为1
            let [_a, mut _b] = _v; // 解构赋值
                                   // _a = 1; // error: _a只读
            _b = 1;

            let mut _v = [0, 0];
            let _a = &mut _v[0];
            *_a = 1; // _v的值变成[1, 0]
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
        let _v = if c {
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
