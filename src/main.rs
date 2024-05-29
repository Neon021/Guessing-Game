use rand::Rng;

fn main() {
    println!("Please guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess");

        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Your guess is less than the secret number"),
            std::cmp::Ordering::Equal => {
                println!("Congrats! Your guess is correct");
                break;
            }
            std::cmp::Ordering::Greater => println!("Your guess is greater than the secret num"),
        }
    }
}
