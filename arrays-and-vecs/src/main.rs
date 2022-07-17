fn fixed_length_arrays<'a>() -> &'a [i32; 4] {
    /*
     * 'a is a lifetime annotation.
     * in this case, it means that the reference returned by the function must not outlive the scope of the function
     * (function scope is global in this case, but more useful in other cases)"""
     */
    let a: [i32; 4] = [1, 2, 3, 4]; //explicit type annotation.
    let b = a; //arrays are value types and are copyable, assuming their members are copyable.
    println!("{:?}, {:?}", a, b);

    let c: &[i32] = &b; //a slice, an array of length not known at compile time.
                        //c is a "fat pointer," two words long and contains the pointer + a payload (the length)
    println!("{}", b[0]);
    println!("{}", c[2]);
    &[1, 2, 3, 4]
}

fn slices(x: &[i32]) {
    let _b = x; //slice of entire array
    let _c = &_b[0..2]; //first two elements.
    let _c = &_b[1..3]; //inclusive 1, exclusive 3.
    let _c = &_b[..3]; //first 3 elements.
    let _c = &_b[1..]; //all elements except the first
    let _c = &_b[..]; //the entire array.
}

fn for_loop(){
    for i in 1..11 {
        print!("{} ", i);
    }
}

//vectors have move semantics, and are heap allocated with an owning reference.

fn vectors(){
    let _v = vec![1,2,3,4]; //initialize w/ "literal"
    let v: Vec<i32> = Vec::new(); //initialize w/ new()
    println!("\ninitial capacity is {}",v.capacity());
    //with_capacity will construct a vector with a given capacity.
    let mut vec = Vec::with_capacity(10);
    assert!(vec.capacity() == 10 && vec.len() == 0);
    for i in 0..10 {
        vec.push(i);
    }
    assert!(vec.len() == 10 && vec.capacity() == 10);
    let val = 4;
    let count = 10;
    println!("initialize a vector of size {} with values {}
     by using vec![{};{}]",count,val,val,count);
    let v = vec![val;count];
    assert!(v.len() == count);
    for value in v {
        assert!(value == val);
    }
}

fn main() {
    let b: &[i32] = fixed_length_arrays(); //b is a reference to a fixed
    slices(b);
    for_loop();
    vectors();
}
