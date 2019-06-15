fn main() {
    {
        let _x:i32; // 使用let声明变量, 其值在初始化后只读
        _x = 0; // 初始化
        // _x = 1; // error code

        let _x = 0; // 使用let声明并初始化变量, 其值只读
        // _x = 1; // error code

        let mut _x = 0; // 使用let mut声明并初始化变量, 其值可写
        _x = 1;
    }

    {
        const _X: i32 = 0; // 使用const声明常量, 必须同时提供常量数据类型, 必须在声明的同时初始化常量值
                          // 只接受常量表达式作为右值, 也就是说函数调用返回值或其它运行时运算值均非法
    }

    {
        let _x = 0;
        let _x = 0.0; // 变量隐藏(Shadow), 常量不能隐藏

        let _x = 0;
        let mut _x = 0.0;
        _x = 1.0;

        let mut _x = 0;
        _x = 1;
        let _x = 0.0;
    }

    {
        {
            let _v_i8 = 0i8; // 8位有符号整型
            let _v_u8 = 0u8; // 8位无符号整型

            let _v_i16 = 0i16; // 16位有符号整型
            let _v_u16 = 0u16; // 16位无符号整型

            let _v_i32 = 0; // 32位有符号整型, 默认类型
            let _v_u32 = 0u32; // 32位无符号整型

            let _v_i64 = 0i64; // 64位有符号整型
            let _v_u64 = 0u64; // 64位无符号整型

            let _v_isize = 0isize; // 根据系统架构定
            let _v_usize = 0usize; // 根据系统架构定

            let _v_integer = 1_024; // 十进制(默认), _作为虚拟分隔符方便阅读
            let _v_integer = 0xFF; // 十六进制
            let _v_integer = 0o77; // 八进制
            let _v_integer = 0b1111_0000; // 二进制
            let _v_integer = b'F'; // 字节, 上面4种写法都可以在值后写上类型以指定变量类型, 而这种写法限制是u8
        }

        {
            let _v_f32 = 0.0f32; // 32浮点数
            let _v_f64 = 0.0; // 64浮点数, 默认类型
        }

        let _v_bool = true; // 布尔类型

        let _v_char = '\0'; // 字符类型, 使用单引号

        { // 复合数据类型: 元组, 元素数据类型和元素数量都是在编译期就能确定的
            let _v_tuple = (); // (匿名)(空)(单位)元组类型

            let mut _v_tuple_t0 = (0, 0.0, true);
            let _v_tuple_t1 = (1, 1.0, false);
            _v_tuple_t0 = _v_tuple_t1; // 元组类型元素一一对应即认为两个元组类型相同

            let mut _v_tuple = (0); // 当元组数据类型只有一个元素时, 元组数据类型将退化成一个普通数据类型
            _v_tuple = 1;

            let _v_tuple = (0, 0.0, false, '\0');
            let (_x, _y, _z, _w) = _v_tuple;
            let _x = _v_tuple.0;
            let _y = _v_tuple.1;
            let _z = _v_tuple.2;
            let _w = _v_tuple.3;
        }

        { // 复合数据类型: 数组, 元素数据类型和元素数量都是在编译期就能确定的
            let _v_array: [i32; 0] = []; //空数组类型
            let _v_array = [true; 2]; // [true, true]
        }

        println!("{}", max(0, 1));

        let v_tuple = (5, 3);
        println!("{}", order_a_tuple(v_tuple).0);
        println!("{}", v_tuple.0);

        let v_array = [5, 2];
        println!("{}", order_a_array(v_array)[0]);
        println!("{}", v_array[0]);
    }
}

// 使用 fn 定义一个函数, -> 指定返回值类型, /// 用于文档注释, // 用于一般注释
/// max 返回两个值中较大那个
fn max(x: i32, y: i32) -> i32 {
    if x > y {
        x //不写 ; 表明这是一个表达式
    } else {
        y
    }
}

// 使用元组作为返回值类型实现返回多个值
/// order_a_tuple 从小到大排序一个拥有两个i32元素的元组并返回
fn order_a_tuple((x, y): (i32, i32)) -> (i32, i32) {
    if x < y {
        (x, y)
    } else {
        (y, x)
    }
}

/// order_a_array 从小到大排序一个拥有两个i32元素的数组并返回
fn order_a_array(array: [i32; 2]) -> [i32; 2] {
    if array[0] < array[1] {
        array
    } else {
        [array[1], array[0]]
    }
}
