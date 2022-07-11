fn main() {
    let _x: bool = true;
    let _x = true;
    let _x = 34;
    let _x = 34isize;
    let _x = 34u8;
    let _x = 34f32;
    let _x = 34i64;

    let _x = 0b1100;
    let _x = 0o14;
    let _x = 0xe;
    let _y = 0b1100_0011;

    let _x = 10 as f32;
    let _x = 4f32 as i32;
    let _x = 400u16 as u8;
    assert_eq!(144, _x);

    /*
    Rust has the following operators:
Type 	Operators
Numeric 	+, -, *, /, %
Bitwise 	|, &, ^, <<, >>
Comparison 	==, !=, >, <, >=, <=
Short-circuit logical 	||, &&

All of these behave as in C++, however, Rust is a bit stricter about
 the types the operators can be applied to - the bitwise operators 
 can only be applied to integers and the logical operators can only 
 be applied to booleans. Rust has the - unary operator which negates 
 a number. The ! operator negates a boolean and inverts every bit on 
 an integer type (equivalent to ~ in C++ in the latter case). Rust has
  compound assignment operators as in C++, e.g., +=, but does not have 
  increment or decrement operators (e.g., ++).
     */
}
