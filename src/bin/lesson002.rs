struct S {
    value: i32,
}

enum E {
    Value(i32),
}

fn main() {
    // 所有权规则:
    // 1. Rust里的每个value都有一个被称为owner的变量
    // 2. 每个value在一个时间点只能有一个owner
    // 3. 当owner离开作用域, value将会被删除

    // 一个数据类型的一个实际值所占用的内存, 只有两种情况, 要么全在栈上, 要么部分在栈上部分在堆里
    // 对一个值进行复制, 仅复制栈上的那部分被称为浅复制, 同时把堆里那部分也复制了被称为深复制
    // 对于一个全在栈上的值, 浅复制与深复制是一样的, 复制后是完全独立的两个值
    // 对于一个部分在栈上部分在堆里的值, 浅复制后，栈上那部分值是完全独立的而堆里那部分数据共享, 深复制才是完全独立的两个值

    // Rust默认只进行浅复制, 而且在复制后把源值标记为无效, 所以Rust中浅复制也被称为移动

    // Rust有一个叫Copy的特征(trait), 所有实现这个特征的类型在其进行值复制时都是深复制, Copy特征与Drop特征互斥
    // 标量(简单)数据类型都是Copy的, 包括: 整型, 浮点型, 布尔型, 字符型
    // 自定义数据类型: 枚举和结构体, 不是Copy的
    // 对于复合数据类型: 元组和数组, 只要元素都是Copy的, 那就是Copy的
    // PS: 在我的理解里, 引用也是Copy的

    // 值的移动发生在三个地方: 赋值, 函数/方法传参, 函数/方法返回值

    // 引用是一种数据类型, 它指向一个值而不获取该值的所有权

    // 使用引用的编程方式被称为借用

    // 引用规则:
    // 1. 在任意时间点, 在同一作用域内, 要么只有一个可写引用, 要么只有多个只读引用
    // 2. 引用必须有效

    let _v_t = (32, 32);
    println!("{}", exchange_tuple(_v_t).0);
    println!("{}", _v_t.0);

    let _v_a = [32, 32];
    println!("{}", exchange_array(_v_a)[0]);
    println!("{}", _v_a[0]);

    {
        let s = S { value: 32 };
        exchange_struct(s);
        // println!("{}", s.value); // error code
    }

    {
        let e = E::Value(32);
        exchange_enum(e);
        // match e {  // error code
        //     E::Value(value) => {
        //         println!("{}", value);
        //     }
        // }
    }

    {
        let s_ref = &S { value: 32 }; // 使用&获取一个值的引用
        println!("{}", s_ref.value);

        exchange_struct_ref(s_ref); // 借用

        {
            let s = S { value: 32 }; // 只读值
            let s_ref = &s; // 只读引用
            println!("{} {}", s.value, s_ref.value);

            let mut s = S { value: 32 }; // 可写值
            s.value = 64;
            let s_ref = &s; // 只读引用
            println!("{} {}", s.value, s_ref.value);
        }

        {
            let mut _v_s = S { value: 32 }; // 只有可写值, 才可以获取可写引用
            let _v_s_ref0 = &mut _v_s;
            // let _v_s_ref1 = &mut s; // error code: 同一作用域内, 对同一个值最多只能有一个可写引用
            // println!("{} {}", _v_s_ref0.value, _v_s_ref1.value);

            let mut _v_i32 = 0;
            let _v_i32_ref0 = &mut _v_i32;
            // let _v_i32_ref1 = &mut _v_i32; // error code
            // println!("{} {}", _v_i32_ref0, _v_i32_ref1);

            {
                let _v_i32_ref0: &i32;

                {
                    let _v_i32 = 0;
                    // _v_i32_ref0 = &_v_i32; // error code: 引用无效了
                }

                // println!("{}", _v_i32_ref0); // error code: 未初始化的值不能使用
            }
        }

        {
            let mut _v_s = S { value: 32 };
            let _v_s_ref0 = &_v_s; // 同一作用域内, 只读引用可以有任意个
            let _v_s_ref1 = &_v_s;
            // let _v_s_ref2 = &mut s; // error code: 但同时存在可写引用
            // println!("{} {} {}", _v_s_ref0.value, _v_s_ref1.value, _v_s_ref2.value);
        }

        {
            let _v_a = [0; 5];
            let _v_s = &_v_a[..];

            let _v_str = String::from("hello world");
            let _v_s = &_v_str[..];
        }
    }
}

fn exchange_tuple(t: (i32, i32)) -> (i32, i32) {
    t
}

fn exchange_array(a: [i32; 2]) -> [i32; 2] {
    a
}

fn exchange_struct(s: S) -> S {
    s
}

#[allow(unused)]
fn exchange_struct_ref(s: &S) {}

fn exchange_enum(e: E) -> E {
    e
}
