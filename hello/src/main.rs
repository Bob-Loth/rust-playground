fn foo(_x: &'static str) -> &'static str {
    "world" + _x
}

fn main() {
    println!("Hello, {}!", foo("bar"));
}
