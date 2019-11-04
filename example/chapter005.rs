/**
 * 结构体, 与元组类似：元素数据类型和元素数量都是在编译期就能确定的
 * 不同的是, 元组无法命名元素, 而结构体可以
 */
fn main() {
    // 初始化一个结构体
    let son0 = Son {
        name: String::from("Link"),
        age: 20,
    };
    println!("{:?}", son0);
    son0.hi();

    let son1 = make_son(String::from("Hugo"));
    println!("{:?}", son1);

    // 初始化一个结构体, 从另一个实例复制值
    let son2 = Son {
        name: String::from("Neway"),
        ..son1
    };
    println!("{:?}", son2);

    // 当变量名与结构体字段名相同时, 可以短写
    let age = 40i8;
    let son3 = Son {
        name: String::from("Zelda"),
        age,
    };
    println!("{:?}", son3);

    let son4 = Son::new(String::from("Hyrule"), 50);
    son4.hi();

    let rect = Rect(10, 10);
    println!("{:?}'s area is {1}!", rect, rect.area());

    let pos = Pos(10, 10);
    println!("{:?}", pos);
}

#[derive(Debug)]
struct Son {
    name: String,
    age: i8,
}

impl Son {
    // 方法, 调用方式: StructInstanceName.MethodName
    fn hi(&self){
        println!("Hi, my name is {0}! {1} years old!", self.name, self.age);
    }
    // 联合函数, 调用方式: StructName::FunctionName
    fn new(name:String, age:i8) -> Son {
        Son {
            name,
            age
        }
    }
}

fn make_son(name: String) -> Son {
    // 当变量名与结构体字段名相同时, 可以短写
    return Son { name, age: 30 };
}

// 对于两个元组类型, 只要两者的元素数量相同, 类型一致且顺序相同, 那两者就是相同的类型
// 如何让两个类型相同的元组成为不同的类型?
// 答案就是: 元组结构体, 其本质上就是给元组类型加上一个命名
#[derive(Debug)]
struct Rect(i32, i32);

impl Rect {
    fn area(&self) -> i32 {
        self.0 * self.1
    }
}

#[derive(Debug)]
struct Pos(i32, i32);

// (空/单位)结构体
struct Empty {}
