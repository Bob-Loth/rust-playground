fn foo(x: i32) -> &'static str {
    let result: &'static str;
    if x < 10 {
        result = "less than 10";
    } else {
        result = "10 or more";
    }
    return result;
}

fn bar(x: i32) -> &'static str {
    if x < 10 {
        "less than 10" //no semicolon indicates return
    } else {
        "10 or more"
    }
}

fn print_all(all: Vec<i32>) {
    for a in all.iter() {
        println!("{}", a);
    }
}

fn print_all2(all: Vec<i32>) {
    for a in 0..all.len() {
        println!("{}", a);
    }
}

fn print_all_keyvalues(all: Vec<i32>) {
    for (i, a) in all.iter().enumerate() {
        println!("{} : {}", i, a);
    }
}

fn double_all(all: &mut Vec<i32>) {
    for a in all.iter_mut() {
        *a += *a;
    }
}

fn print_some(x: i32) {
    match x {
        0 | 2 => println!("x is zero or two"),
        10 => println!("x is ten"),
        y if y < 20 => println!("x is less than 20: {}",y) ,
        y => println!("x is something else {}", y),
    }
}

fn main() {
    let mut x = 0;
    while x <= 20 {
        print_some(x);
        x += 1;
    }
}
