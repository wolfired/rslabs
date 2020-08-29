pub fn trace(head:&str, head_wid:usize, body:&str, separator:Option<&str>) {
    let mut s = ":\t";
    if let Some(separator) = separator {
        s = separator;
    }
    println!("{0:>1$}{3}{2}", head, head_wid, body, s);
}
