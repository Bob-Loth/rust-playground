use crate::Expr::{Add, Lit, Mul};
use std::fmt;

#[derive(Debug)]
struct AnotherStruct {
    s: &'static str,
}

struct S {
    field1: i32,
    field2: AnotherStruct,
}

fn structs() {
    let a_struct = S {
        field1: 43,
        field2: AnotherStruct { s: "hello" },
    };
    println!("{}, {}", a_struct.field1, a_struct.field2.s);
}

fn compare_struct(s1: S, s2: &S) {
    let f = s1.field1;
    if f == s2.field1 {
        println!("field1 matches!");
    }
}

struct _Empty;
fn _empty() {
    let _e = _Empty;
}

fn tuple(s: AnotherStruct) -> (i32, i32, AnotherStruct) {
    (3, 2, s)
}
struct Point(f32, f32);
fn tuple_struct(p: Point) {
    //a tuple argument. This syntax is not very common
    let Point(a, b) = p; //tuple destructuring.
    println!("p was ({}, {})", a, b);
}

enum E {
    E1,
    E2,
}

//usually just annotate with a debug print #[derive(Debug)]
impl fmt::Debug for E {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            E::E1 => write!(f, "1"),
            E::E2 => write!(f, "2"),
        }
    }
}
// C++ has variant<T>, which is similar to Rust's enums.
fn some_enums() {
    let x: E = E::E2;
    match x {
        E::E1 => println!("{:?}", x),
        E::E2 => println!("E3"),
    }
}

enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Lit(i32),
}

fn eval(e: Box<Expr>) -> i32 {
    match *e {
        Add(l, r) => eval(l) + eval(r),
        Mul(l, r) => eval(l) * eval(r),
        Lit(x) => x,
    }
}

fn arithmetic_expressions() {
    let x = Box::new(Mul(Box::new(Lit(2)), Box::new(Lit(4))));

    let y = Box::new(Lit(10));
    let z = Box::new(Add(x, y));
    println!("{}", eval(z));
}

fn options() -> &'static str {
    let x: Option<i32> = Some(2);
    let _y: Option<i32> = None;
    match x {
        Some(..) => "has something",
        _ => "has nothing",
    }
}

struct S1 {
    field1: i32,
    field2: S2,
}
struct S2 {
    field: i32,
}

struct S3 {
    f: i32,
}
struct S4<'a> {
    f: &'a mut S3, // mutable reference field
}
struct S5<'a> {
    f: &'a S3, // immutable reference field
}

fn inherited_mutable_reference() {
    let mut s3 = S3 { f: 56 };
    let s4 = S4 { f: &mut s3 };
    s4.f.f = 45; // legal even though s2 is immutable
                 // s2.f = &mut s1; // illegal - s2 is not mutable
    let s3 = S3 { f: 56 };
    let mut s5 = S5 { f: &s3 };
    s5.f = &s3; // legal - s3 is mutable
                // s3.f.f = 45; // illegal - s3.f is immutable
}

fn inherited_mutability() {
    let _s = S1 {
        field1: 45,
        field2: S2 { field: 23 },
    };
    // s is deeply immutable, the following mutations are forbidden
    // s.field1 = 46;
    // s.field2.field = 24;

    let mut s = S1 {
        field1: 45,
        field2: S2 { field: 23 },
    };
    // s is mutable, these are OK
    s.field1 = 46;
    s.field2.field = 24;
}

//Cell and RefCell allow immutable types to have parts that are mutable.
fn cell_and_refcell() {
    use std::{cell::RefCell, rc::Rc};
    struct S {
        field: i32,
    }
    fn foo(x: Rc<RefCell<S>>) {
        {
            let s = x.borrow();
            println!("the field, twice {}, {}", s.field, x.borrow().field);
            //let s = x.borrow_mut(); // Error - we've already borrowed the contents of x
        }
        let mut s = x.borrow_mut();
        s.field = 41;
        // println!("The field {}", x.borrow().field); // Error - can't mut and immut borrow
        println!("The field {}", s.field);
    }

    let s = S { field: 12 };
    let x: Rc<RefCell<S>> = Rc::new(RefCell::new(s));
    foo(x.clone());
    println!("The field {}", x.borrow().field);
}

fn main() {
    structs();
    compare_struct(
        S {
            field1: 9,
            field2: AnotherStruct { s: "hello" },
        },
        &S {
            field1: 9,
            field2: AnotherStruct { s: "2" },
        },
    );
    println!("{:?}", tuple(AnotherStruct { s: "hello" }));
    tuple_struct(Point(4.0, 5.4));
    some_enums();
    arithmetic_expressions();
    options();
    inherited_mutability();
    inherited_mutable_reference();
    cell_and_refcell();
    println!("{:?}", E::E1);
}
