//! std::fmt
//! 格式化与打印字符串相关工具定义在这个模块

fn main() {
    println!("参数引用:");
    println!("{}", format!("Hello"));
    println!("{}", format!("Hello {}", "world"));
    println!("{}", format!("The number is {}", 1));
    println!("{}", format!("{:?}", (0,0)));
    println!("{}", format!("value is {value}", value = 4));
    println!("{}", format!("{} {}", 0, 1));
    println!("{}", format!("{:04}", 1));
    println!("{}", format!("{2} {1} {0} {} {} {}", 0, 1, 2));
    println!("{}", format!("{c} {b} {a} {} {} {}", a = 0, b = 1, c = 2));

    // 宽度, 下面5打印效果相同
    println!("\n指定宽度:");
    println!("{}", format!("|{:8}|", "x"));
    println!("{}", format!("|{0:8}|", "x"));
    println!("{}", format!("|{:1$}|", "x", 8));
    println!("{}", format!("|{1:0$}|", 8, "x"));
    println!("{}", format!("|{:width$}|", "x", width = 8));

    // 填充/对齐, 默认填充字符是' ', 非数字字符串默认左对齐, 数字字符串默认右对齐
    println!("\n填充/对齐(非数字字符串):");
    println!("{}", format!("|{:8}|", "x")); //默认左对齐, 默认填充字符' '
    println!("{}", format!("|{:<8}|", "x")); //指定左对齐, 默认填充字符' '
    println!("{}", format!("|{:>8}|", "x")); //指定右对齐, 默认填充字符' '
    println!("{}", format!("|{:-<8}|", "x")); //指定左对齐, 指定填充符'-'
    println!("{}", format!("|{:-^8}|", "x")); //指定居中对齐, 指定填充符'-'
    println!("{}", format!("|{:->8}|", "x")); //指定右对齐, 指定填充符'-'

    println!("\n填充/对齐(数字字符串):");
    println!("{}", format!("|{:8}|", 1)); //默认右对齐, 默认填充字符' '
    println!("{}", format!("|{:>8}|", 1)); //指定右对齐, 默认填充字符' '
    println!("{}", format!("|{:<8}|", 1)); //指定左对齐, 默认填充字符' '
    println!("{}", format!("|{:->8}|", 1)); //指定右对齐, 指定填充字符'-'
    println!("{}", format!("|{:-^8}|", 1)); //指定居中对齐, 指定填充字符'-'
    println!("{}", format!("|{:-<8}|", 1)); //指定左对齐, 指定填充字符'-'

    // (正负)符号/#/0
    println!("\n(正负)符号/#/0:");
    println!("{}", format!("|{:8}|", 1)); // 默认正号不显示
    println!("{}", format!("|{:+8}|", 1)); // 强制显示正负号
    println!("{}", format!("|{:+8}|", 1)); // 强制显示正负号
    println!("{}", format!("|{:#?}|", (0,1))); // 优化调试格式
    println!("{}", format!("|{:#b}|", 1)); // 0b二进制式
    println!("{}", format!("|{:#o}|", 1)); // 0o八进制式
    println!("{}", format!("|{:#x}|", 15)); // 0x十六进制式(小写)
    println!("{}", format!("|{:#X}|", 15)); // 0x十六进制式(大写)

    // 精度
    println!("\n精度:");
    println!("{}", format!("|{:8.4}|", 0.123456789)); // 对于小数, 显示小数点后指定位数, 进行四舍五入
    println!("{}", format!("|{:8.4}|", "0.123456789")); // 对于字符串, 显示指定数量字符
    println!("{}", format!("|{:8.4}|", "中华人民共和国")); // 对于字符串, 显示指定数量字符
    println!("{}", format!("|{:-^8.1$}|", "中华人民共和国", 4)); // 对于字符串, 显示指定数量字符
    println!("{}", format!("|{:-^8.*}|{:-^8.*}|", 4, "中华人民共和国", 6, "中华人民共和国")); // 对于字符串, 显示指定数量字符

    // 转义
    println!("\n转义:");
    println!("{}", format!("{{{}}}", 0));

    // 格式化特性
    println!("\n格式化特性:");
    trace("Display", 32, format!("{:}", 0).as_str());
    trace("Debug", 32, format!("{:?}", (0,0xf0f0)).as_str());
    trace("Debug, lower hex", 32, format!("{:x?}", (0,0xf0f0)).as_str());
    trace("Debug, upper hex", 32, format!("{:X?}", (0,0xf0f0)).as_str());
    trace("Binary", 32, format!("{:b}", 0b1010).as_str());
    trace("Octal", 32, format!("{:o}", 0o7070).as_str());
    trace("LowerHex, no prefix '0x'", 32, format!("{:x}", 0xf0f0).as_str());
    trace("UpperHex, no prefix '0x'", 32, format!("{:X}", 0xf0f0).as_str());
    trace("LowerExp", 32, format!("{:e}", 0).as_str());
    trace("UpperExp", 32, format!("{:E}", 0).as_str());
    trace("Pointer", 32, format!("{:p}", &0).as_str());
}

fn trace(head:&str, head_wid:usize, body:&str) {
    println!("{0:>1$}:\t{2}", head, head_wid, body)
}
