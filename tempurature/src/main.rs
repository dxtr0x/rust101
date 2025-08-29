use std::{io::Write, thread::sleep, time::Duration};

fn input() -> Result<f64, std::num::ParseFloatError>{
    let mut number : String = String::new();
    std::io::stdin()
    .read_line(&mut number)
    .unwrap();

    // Don't forget .trim() in input function;
    number.trim().parse::<f64>()
}

fn choice() -> i8{
    println!("Enter the choice :  ");
    println!("1 - C2F ");
    println!("2 - F2C ");
    print!("Choice : ");
    let _ = std::io::stdout().flush();
    loop{
        let num_choice: i8 = input().unwrap() as i8;
        if num_choice == 2 || num_choice == 1 {
            return num_choice;
        }
    }
}

fn c2f(corf : i8){
    print!("{}",if corf == 1 {"Enter C :" } else {"Enter F :"});
    let _ = std::io::stdout().flush();
    let dgr :f64 = input().unwrap();
    let num = if corf == 2 {
            (5.0 / 9.0) * (dgr - 32.0)
        } else {
            (9.0 / 5.0) * dgr + 32.0
        };
    println!("\x1b[1;32m {} is :{}\x1b[0m",if corf == 1 {" > C2F " } else {" > F2C "},num);
}


fn Tempurature(){
    loop {
        println!("\x1b[41m =============== Tempurate Convertor ============== \x1b[0m");
        let num : i8 = choice();
        c2f(num);
        sleep(Duration::from_secs(10));
        for _ in 0..7{
            print!("\r\x1b[2K\x1b[A");
        }
    }
}
