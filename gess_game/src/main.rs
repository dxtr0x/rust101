use std::io::{self, Write};

fn input() -> i32 {
    let mut num = String::new();
    print!("Enter your guess (1-10) : ");
    io::stdout()
        .flush()
        .unwrap();
    io::stdin()
        .read_line(&mut num)
        .unwrap();

    num.trim()
        .parse::<i32>()
        .unwrap()
}

fn guesse_game(){
    loop{
        // To use rand lib 0.9 version we use rand::random_range(range ex: 1..=10);
        let guess = rand::random_range(1..=10);
        let num = input();
        println!("Your input is : {} \nYour Guesse number : {}\n{}",
                    &num,
                    &guess,
                    if num == guess {"You Win\n" } else { "You Lose\n" });
    }
}

fn main() {
    guesse_game();
}
