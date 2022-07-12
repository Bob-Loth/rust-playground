fn reference_types() {
    let x = &3; //a reference to a constant value. (3)
    let y = *x; //a mutable value, initialized from x's value of 3
    let z = &y; // a borrowed reference to y
                //y += 1; //cannot do this, immutable by default

    println!("y={}, z={}", y, z);
    reftypes(x, *x);
    reftypes(&y, y);
}

fn reftypes(_reference: &i32, _value: i32) {
    //empty, used to enforce/annotate types
}

fn add() -> fn(x: &mut i32) {
    //returns a function
    fn addone(x: &mut i32) {
        *x += 1;
    }
    addone
}

fn borrowing() {
    let mut six = 6;
    let _seven = 7; //redefine as not mutable and the mutable ref will break
    println!("before addone: {}", six);
    let addone = add();
    addone(&mut six); //mutable reference. Must be explicit.
                      //addone(&mut _seven); //Cannot make a mutable reference from an immutable value.
    println!("after addone: {}", six);
}

fn reference_mutability() {
    let mut x = 5;
    let mut y = 2;
    x += 1;
    let _xr = &mut x;
    //xr = &mut x; not legal, xr (the reference) is immutable.

    let mut _xr = &mut x; //redefine xr as mutable (somewhat bad practice)
    _xr = &mut y;

    let x = 5;
    let y = 6;
    let mut _xr = &x;
    _xr = &y; //xr is mutable, y is not.
}

//This illustrates the borrowing concept. Data can only be modified via one variable or pointer.
//If a mutable value is borrowed, it becomes immutable to the duration of the borrow.
//Once the borrowed pointer goes out of scope, it may once again be modified.
fn borrowing_2() {
    let mut _x = 5;
    {
        //scope
        let _y = &_x;
        println!("_x inside inner scope BEFORE: {}", _x);
        _x = 4; //x cannot be assigned while it is borrowed by y.
                //println!("{}", _y); //if y isn't used, it's optimized away. This prevents this and triggers the error.
        println!("_x inside inner scope AFTER: {}", _x);
    }
    _x = 4;
}

fn printref(x: &i32) {
    println!("foo: {}", x);
}
//pointer types will automatically be converted to a reference.
fn bar(x: i32, y: Box<i32>) {
    printref(&x);
    printref(&y);
}

fn scope() {
    let x = 5;
    let xr = &x;
    {
        let _y = 6;
        //xr = &y; //not valid. xr lives longer than what it points to (y)
    }
    println!("{:?}", xr);
}
fn main() {
    reference_types();
    borrowing();
    reference_mutability();
    borrowing_2();
    scope();
    printref(&4);
    let x = 2;
    bar(x, Box::new(x));
}
