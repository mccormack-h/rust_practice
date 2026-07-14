use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_num = rand::thread_rng().gen_range(1..=100);

    loop
    {
        println!("Please input your guess:");

        let mut num = String::new();


        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read line.");

        let num: u32 = match num.trim().parse()
        {
            Ok(number) => number,
            Err(_) => continue,
        };

        println!("You guessed: {num}");

        match num.cmp(&secret_num)
        {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => 
            {
                println!("You win!");
                break;
            }
        }
    }
}