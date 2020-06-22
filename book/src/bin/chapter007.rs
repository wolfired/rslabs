/**
 * 使用包, 箱, 模块管理增长中的项目(Managing Growing Projects with Packages, Crates, and Modules)
 *
 * 当你编写大型程序时, 如何组织你的代码将变得非常重要, 因为在你的脑海里跟踪整个程序是很困难的.
 * 通过分组相关功能, 按特性分割代码, 你可以直观的知道在那能找到实现某特性的代码, 知道在那能修改这个实现.
 *
 * 到目前为止我们写的程序都只有一个文件一个模块. 随着项目的增长你可以通过分割多个模块多个文件来组织你的代码.
 * 一个包(Package)可以包含多个二进制箱(Binary Crate)和一个可选的库箱(Library Crate).
 * 随着包的增长, 你可以提取部分到一个独立的箱, 让它成为一个外部依赖. 本章涵盖这些技术.
 */
fn main() {
    println!("https://doc.rust-lang.org/book/");
}
