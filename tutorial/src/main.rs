fn main() {
    // rust implicitly determines type at compile time
    let x = 4;
    // mutable variable
    let mut z = 9;

    // explicit type definition
    let y: u32 = 6;

    println!("x is {}", x);
    println!("y is {}", y);

    // this will throw an error - x is not mutable
    // x = 7;

    // however, this will not!
    // you can completely recreate x
    let x = 5;

    // this is okay because z was originally declared as mutable
    z = 10;
    
    println!("x is {}", x);
    println!("z is {}", z);
}
