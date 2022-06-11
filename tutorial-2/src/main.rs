fn main() {
    // rust implicitly determines type of let vars at compile time
    let x = 4;
    println!("x is {}", x);

    // create a scope
    {
        // this definition overrides the parent scope
        // ...within this scope
        let x = 2;
        println!("x is {}", x);
    }

    // this will not throw an error!
    // you can completely recreate x
    let x = x + 5;
    
    println!("x is {}", x);

    // constants
    // with constant variables, there is no implicit type definition
    // you must state one at declaration like below
    const SECONDS_IN_MINUTE: u32 = 60;
    println!("SECONDS_IN_MINUTE is {}", SECONDS_IN_MINUTE);
}