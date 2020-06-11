fn main() {
    println!("Hello, world!\n");
    another_function();
    math_function(2, 10);
    let x = five();
    let z = plus_one(5);

    println!("The value of x is: {}", x);
    println!("The value of z is: {}", z);
}

fn another_function(){
    println!("Outside main.\n");
}

fn math_function(x:i32, y:i32){
    println!("The value passed is: {}", x);
    println!("The second value passed is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x:i32) -> i32{
     x + 1
}
