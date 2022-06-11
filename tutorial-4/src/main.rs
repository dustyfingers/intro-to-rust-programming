fn main() {
    // primitive data types

    // scalar types
    // ex: integer, boolean

    // types of integers
    // signed means we can use the negative sign
    // i8, i16, i32, i64, i128
    let x: i32 = -2;

    // the number value refers to the number of bits available to the value
    // eg an i8 can store up to 2^8 in positive values (pretty much)
    // and a u8 can store 2^8 values, but allowing negative values and centered at 0 (-128 to 128)
    
    // unsigned means we cannot use negative values
    // u8, u16, u32, u64, u128
    let z: u128 = 123154125132432;

    // types of floating point values
    // f32 (single-precision), f64 (double-precision)
    
    // default type is f64, double precision
    let floating_point_value = 10.92;
    let single_precision:f32 = 33.4;

    // booleans
    // true or false

    // char type - it MUST USE single quotes
    let letter: char = 'x';
    let letter_too = 'z';

    // compound types
    // ex: array, tuple
    
    // tuples
    let tup1: (i32, bool, char) = (1, true, 'f');
    let mut tup2: (i8, bool, char) = (2, false, 'j');

    tup2.2 = 'g';

    // arrays
    // all elements in an array must be of the same type
    let arr = [1,2,3,4,5];
    let mut mutable_arr = [1,2,3,4,5];

    // to be more explicit with the typing of your array:
    let specific_arr: [i32;3] = [1, 0, 1];

    mutable_arr[4] = 3;


    println!("{}", x);
    println!("{}", z);
    println!("{}", floating_point_value);
    println!("{}", single_precision);
    println!("{}", letter);
    println!("{}", letter_too);
    println!("{}", tup1.0);
    println!("{}", tup2.2);
    println!("{}", arr[0]);
    println!("{}", mutable_arr[4]);
    println!("{}", specific_arr[0]);
}
