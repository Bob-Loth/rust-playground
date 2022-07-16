fn fixed_length_arrays(){
    let a: [i32;4] = [1,2,3,4]; //explicit type annotation.
    let b = [1,2,3,4];
    println!("{:?}, {:?}",a, b);
}

fn main() {
    fixed_length_arrays();
}
