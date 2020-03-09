/**
 * 泛型, 特性
 */
fn main() {
    let mut x = &mut Son { age: 0 };
    x.age = 1;
    let y = &mut Son { age: 2 };
    x = y;
    println!("{:?}", x);

    let son = &mut Son { age: 10 };
    say(son);
    let sun = &mut Sun {};
    say(sun);
}

fn say(who: &mut impl ISay) {
    who.say();
}

trait ISay {
    fn say(&mut self) {
        println!("Hi");
    }
}

#[derive(Debug)]
struct Son {
    age: i8,
}

impl ISay for Son {
    fn say(&mut self) {
        println!("Hi, Son");
    }
}

struct Sun {}

impl ISay for Sun {
    // fn say(&self) {
    //     println!("Hi, Sun");
    // }
}
