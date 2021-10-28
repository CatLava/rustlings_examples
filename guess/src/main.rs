use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    println!("enter a guess");

    let secret_number = rand::thread_rng().gen_range(1,101);

    println!("Your secret number is: {}", secret_number);

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        }

        println!("you guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("Winner!");
                break;
                }
            }
        }
}
