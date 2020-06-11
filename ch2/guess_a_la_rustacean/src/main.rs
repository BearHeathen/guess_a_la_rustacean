use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {

     
    // TODO Give a menu with choices of number ranges

        println!("\nGuessing Game v.0.1.0");
        println!("=====================\n");
        println!("\nYou have 10 chances to guess the right number\n");

        let  secret_num = rand::thread_rng().gen_range(1, 101);

        // My own functionality.
        let mut guess_count = 0;

       // println!("The secret number is: {}", secret_num);
        
    loop {
        println!("Please input your guess: ");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
        .expect("Failed to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        

        println!("You guessed {}!", guess);

        match guess.cmp(&secret_num){
            Ordering::Less => {
                println!("Too small!");
               guess_count += 1;
            }
            Ordering::Greater => {
                println!("Too big!");
                guess_count += 1;
            }
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        if guess_count >= 10{
            println!("You've exceeded the maximum number of allowed guesses. Sorry!\n");
            break;
            // TODO Ask to play again.
        }
    }
        

}
