use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=50);

    loop {

        let mut guess = String::new();

        //Le pido un dato al usuario
        println!("Please input your guess:");

        //Incluyo el dato del usuario a la variable guess
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        //Comparo la variable guess con secret_number y veo cual de las 3 variantes coincido a través de match
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

    }

}