struct T<A: ?Sized> {
    arr: A,
}

fn main() {
    let t = T { arr: [0] };
    let rt: &T<[i32]> = &t;
    let tt = &t;

    println!("{}", std::mem::size_of_val(&rt));

    let p = &rt as *const &T<[i32]> as *const usize;
    unsafe {
        println!("{:#x}", *(p.add(1)));
    }

    let s = "你";
    let pp = &s as *const &str as *const usize;
    unsafe {
        println!("{:#x}", *(pp.add(1)));
    }
}
