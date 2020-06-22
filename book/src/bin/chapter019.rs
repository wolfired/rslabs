/**
 * 高级功能(Advanced Features)
 */
fn main() {
    section002();
}

fn section002() {
    trait ISun {
        fn hi(&self);
        fn say() {
            println!("ISun -> say"); //CAN I CALL THIS WITH SUN?
        }
    }

    struct Sun;

    impl Sun {
        fn hi(&self) {
            println!("Sun -> hi");
        }
        fn say() {
            println!("Sun -> say");
        }
    }

    impl ISun for Sun {
        fn hi(&self) {
            println!("ISun -> hi");
        }
        fn say() {
            println!("Sun -> ISun -> say");
        }
    }

    let sun = Sun;

    sun.hi();
    ISun::hi(&sun);

    Sun::say();
    <Sun as ISun>::say();
}
