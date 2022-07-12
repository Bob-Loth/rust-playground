fn raw_pointer() {
    let mut x = 5;
    let x_ptr: *mut i32 = &mut x; //create a raw mutable pointer to x
    println!("x+5={}", add_5(x_ptr));
}

fn add_5(x_ptr: *mut i32) -> i32 {
    unsafe {
        if !x_ptr.is_null() {
            *x_ptr + 5 //return
        } else {
            -1
        }
    }
}

fn main() {
    raw_pointer();
}
