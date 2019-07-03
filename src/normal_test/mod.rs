
use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn guess_number() {
    println!("Guess a 1-100 number!");
    println!("Random a bingo number......");
    let bingo = rand::thread_rng().gen_range(1, 101);
    println!("Random number is created!");

    loop {
        println!("Please guess a number:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("error~");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess {}", guess);
        match guess.cmp(&bingo) {
            Ordering::Less => println!("Your guess is too small!"),
            Ordering::Greater => println!("Your guess is too big!"),
            Ordering::Equal => {
                println!("Your guess is bingo!");
                println!("Game Over");
                break;
            }
        }
    }
}