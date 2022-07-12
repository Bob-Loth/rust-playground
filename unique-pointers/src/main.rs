fn foo() -> Box<i32> {
    let x = Box::new(55);
    x //move semantics
}

fn main() {
    let x = Box::new(75);
    //const auto x = std::make_unique<const int>(75);
    let z = foo();
    //x = y not allowed, neither is *x = *y.

    println!("'x' points to {}", *x);
    println!("'z' points to {}", *z);
}
