// Example custom build script.
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=c/sayhi.c");
    // Use the `cc` crate to build a C file and statically link it.
    cc::Build::new()
        .include("c")
        .file("c/sayhi.c")
        .compile("sayhi");
}
