fn main() {
    // Scalar Types
    // A scalar type represents a single value.
    // Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

    // Integer Types
    // signed integer types start with i instead of u
    // Length	Signed	Unsigned
    // 8-bit	i8	    u8
    // 16-bit	i16 	u16
    // 32-bit	i32	    u32
    // 64-bit	i64	    u64
    // 128-bit	i128	u128
    // arch	    isize	usize
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("this is a signed integer: {}", guess);

    // Floating-Point Types
    // Rust also have a 2 primitive types for floating-point numbers
    // which are numbers with decimal points. Rust’s floating-point types are f32 and f64
    let x = 2.2; // f64
    let y: f32 = 3.5; // f32
    println!("this is a floating-point number: {}", x + y);

    // Numeric Operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    // The Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // The Array Type
    let a = [1, 2, 3, 4, 5];
}
