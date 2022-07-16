fn destructuring_1(pair: (i32, i32)) {
    match pair {
        (x, y) => {
            println!("{}, {}", x, y);
        }
    }
}

fn destructuring_2((x, y, z): (f32, f32, f32)) {
    println!("Point: ({}, {}, {})", x, y, z);
}

struct St {
    f1: i32,
    f2: f32,
}

enum En {
    Var1,
    Var2,
    Var3(i32),
    Var4(i32, St, i32),
}

trait Type {
    fn type_of(&self) -> &'static str;
}

impl Type for En {
    fn type_of(&self) -> &'static str {
        use En::*;
        match self {
            &Var1 => "Var1",
            &Var2 => "Var2",
            &Var3(..) => "Var3",
            &Var4(..) => "Var4",
        }
    }
}

fn destructuring_3(x: &En) {
    use En::*;
    match x {
        &Var1 => println!("first variant"),
        &Var2 => println!("second variant"),
        &Var3(5) => println!("third variant with argument 5"),
        &Var4(3, St { f1: 3, f2: x }, 45) => {
            println!("direct access of struct values using destructuring: {}", x)
        }
        _ => println!("undefined, but is of type {}", x.type_of()),
    }
}

struct Big {
    _field1: i32,
    _field2: i32,
    field3: i32,
    _field4: i32,
    _field5: i32,
    field6: i32,
    _field7: i32,
    _field8: i32,
    _field9: i32,
}

fn elision(b: Big) {
    let Big { field6, field3, .. } = b;
    println!("pulled out field6: {} and field3: {}", field6, field3);
}

struct RefInt {
    field: &'static i32,
}

fn ref_keyword(x: RefInt, b: Big) {
    let RefInt { field: &y } = x;
    println!("x = {}, y = {}", x.field, y);

    let Big {
        field3: ref x,
        ref field6,
        ..
    } = b;
    println!("destructured using ref keyword: {}, {}", x, *field6);
}


fn semantics() {
    semantics_1();
    semantics_2();
}

fn semantics_1() {
    let x = 7i64;
    //direct type
    let y = x;
    println!("x = {} at {:p}, y = {} at {:p}", x, &x, y, &y);
    //copied
    let x = Box::new(6i32);
    let y = x;
    //x is moved to y and x is now inaccessible
    println!("{}", y);
    //cannot print x, throws error
}

fn semantics_2() {
    let x = &En::Var1;
    match *x {
        // Option 1: deref here.
        En::Var1 => {}
        En::Var2 => {}
        En::Var3(..) => {}
        _ => {}
    }
    match x {
        // Option 2: 'deref' in every arm.
        &En::Var1 => {}
        &En::Var2 => {}
        &En::Var3(..) => {}
        &En::Var4(..) => {}
    }
}

fn main() {
    destructuring_1((5, 6));

    let point = (4.5, 2.1, -1.0);
    destructuring_2(point);

    let w = En::Var4(3, St { f1: 3, f2: -0.7 }, 44);
    let x = En::Var1;
    let y = En::Var2;
    let z = En::Var3(5);
    let v = vec![x, y, z, w];
    v.iter().for_each(destructuring_3);

    elision(Big {
        _field1: 1,
        _field2: 4,
        field3: 8,
        _field4: 12,
        _field5: 16,
        field6: 20,
        _field7: 24,
        _field8: 28,
        _field9: 32,
    });

    ref_keyword(
        RefInt { field: &1 },
        Big {
            _field1: 0,
            _field2: 0,
            field3: 0,
            _field4: 0,
            _field5: 0,
            field6: 0,
            _field7: 0,
            _field8: 0,
            _field9: 0,
        },
    );

    semantics();
}
