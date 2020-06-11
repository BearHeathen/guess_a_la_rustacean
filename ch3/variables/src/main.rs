fn main() {

    // Usage of constants.
    const MAX_POINTS: u32 = 100_000;

    println!("The value of the constant MAX_POINTS IS: {}", MAX_POINTS);

    // Usage of mut: without mut this will not compile.
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Usage of "shadowing"
        // Use "let" to shadow a variable
    
    let y = 7;
    let y = y + 1;
    let y = y * 2;

    println!("The value of y is: {}", y);

    //Tuple

    let tup:(i32, f64, u8) = (500, 6.4, 1);

    let(a, b, c ) = tup;

    println!("The value of b is: {}", b)
}
