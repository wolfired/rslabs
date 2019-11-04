use std::fs;

/**
 * 使用fs模块
 */
fn main() {
    let r = fs::read("/home/link/workspace_git/rslabs/LICENSE");
    let bs = r.unwrap();
    println!("{}", bs.len());
}
