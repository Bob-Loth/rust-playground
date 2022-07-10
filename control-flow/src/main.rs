fn foo(x: i32) -> &'static str {
    let result: &'static str;
    if x < 10 {
        result = "less than 10";
    }
    else {
        result = "10 or more";
    }
  return result
}

fn bar(x: i32) -> &'static str {
    if x < 10 {
        "less than 10" //no semicolon indicates return
    } else {
        "10 or more"
    }
}

fn main() {
    let mut x = 9;
    while x <= 10 {
        println!("{}", foo(x));
        println!("{}", bar(x));
        x += 1;
    }
}
