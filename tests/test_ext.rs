#![feature(phase)]
#![promote_variants]

#[phase(plugin)] extern crate enum_ns;

#[deriving(Show)]
enum MyEnum {
    Variant1,
    Variant2,
    Variant3,
}

#[test]
fn print_variants() {
    println!("{}, {}, {}", Variant1, Variant2, Variant3);
}


