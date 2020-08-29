#![allow(dead_code)]

use stdoor::trace;

/// std::env
/// "环境(变量)"模块, 检查和管理进程的"环境(变量)"
/// 这个模块包含了用于检查 "环境变量" "进程参数" "当前目录" 和 "各种重要目录" 的函数。
/// 本模块有些函数和结构体有一个以 "os" 或 "Os" 结尾的版本, 这些特殊版本会返回 "OsString"
/// 而普通版本会返回 "String"

fn main() {
    all_consts();
}

fn all_consts() {
    trace("CPU architecture", 64, std::env::consts::ARCH, None);
    trace("shared library extension, without dot", 64, std::env::consts::DLL_EXTENSION, None);
    trace("shared library prefix", 64, std::env::consts::DLL_PREFIX, None);
    trace("shared library suffix, with dot", 64, std::env::consts::DLL_SUFFIX, None);
    trace("executable binary extension, without dot", 64, std::env::consts::EXE_EXTENSION, None);
    trace("executable binary suffix, with dot", 64, std::env::consts::EXE_SUFFIX, None);
    trace("operating system family", 64, std::env::consts::FAMILY, None);
    trace("operating system", 64, std::env::consts::OS, None);
}
