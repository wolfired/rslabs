#![allow(unused_variables)]
#![allow(dead_code)]
/**
 * 枚举
 */
fn main() {
    let k = IPAddrKind::V4;
    println!("{:?}", k);

    let local = IPAddr::V4(127,0,0,1);
    println!("{:?}", local);

    match local {
        IPAddr::V4(a,b,c,d) => {

        },
        IPAddr::V6(s) => {

        },
    }

    let x = 8i8;
    match x {
        8 => {
            println!();
        },
        _ => {

        },
    }
}

#[derive(Debug)]
enum IPAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IPAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}