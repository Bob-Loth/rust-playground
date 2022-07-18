fn closures_1(y: i32) -> impl Fn(i32) -> i32 {
    //FnOnce is used for functions that should only be called once, FnMut are objects with mutated state.
    let z = 4;
    //like closures in other languages, closures capture their enclosing environment (y and z).
    let f = {
        move |x: i32| {
            x + y + z
        }
    }; //move is used to copy y and z by value, as they are owned by closures_1 function
    f
}

fn add_42(x: i32) -> i64 {
    x as i64 + 42 //cast x to wider type then add.
}

fn generics<F>(f: F) -> i64
    where F: Fn(i32) -> i64
{
    f(0)
}

fn main() {
    let y = 6;
    let f = closures_1(y);
    let x = 1;
    println!("{}", f(x));

    let mut v: Vec<i32> = vec![1, 2, 3];
    v.iter_mut().for_each(|v| *v += f(x));
    println!("{:#?}", v);

    let a = add_42;
    println!("{}",generics(a));
    println!("{}",generics(|x| (x + y).into()));
}
