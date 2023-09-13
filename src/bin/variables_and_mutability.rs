fn main() {
    // incorrectly declared as immutable
    // let x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    // correctly declared as mutable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    // shadowing !== mutability
    let z = 5;

    let z = z + 1;

    {
        let z = z * 2;
        println!("The value of z in the inner scope is: {z}");
    }

    println!("The value of z is: {z}");

    // Difermce between shadowing and mutability

    //this works because we implement shadowing and this changes the type of the variable
    let spaces = "   ";
    let spaces = spaces.len();

    //this doesn't work because we implement mutability and this doesn't change the type of the variable
    // let mut spaces = "   ";
    // spaces = spaces.len();

    println!("The value of spaces is: {spaces}")
}
