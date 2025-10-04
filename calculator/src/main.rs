use std::{io::Write};


fn input() -> Result<f64, std::num::ParseFloatError>{
    let mut number : String = String::new();
    std::io::stdin()
    .read_line(&mut number)
    .unwrap();

    // Don't forget .trim() in input function;
    number.trim().parse::<f64>()
}
enum Operation{
    Plus = 0,
    Minus = 1,
    Multiple = 2,
    Divide = 3
}

impl Operation{
    fn calc(x : f64 , y : f64 , op : Operation) -> f64{
        match op{
            Operation::Plus => x + y,
            Operation::Minus => x - y,
            Operation::Multiple => x * y,
            Operation::Divide => x / y 
        }
    }
}

fn calculatexy(){
    print!("Enter the x value : ");
    std::io::stdout()
        .flush().unwrap();
    let x : f64 = input().unwrap();

    // ============================

    let op : Operation;

    loop{
        print!("Enter the operation (+[0],-[1],*[2],/[3]) : ");
        std::io::stdout()
            .flush().unwrap();
        let tmp  = input().unwrap();
        match tmp as i32 {
            0 => {op = Operation::Plus ; break;} ,
            1 => {op = Operation::Minus ; break;},
            2 => {op = Operation::Multiple ; break;},
            3 => {op = Operation::Divide ; break;},
            _ => continue
        }
    }
    
    // =============================

    print!("Enter the y value : ");
    std::io::stdout()
        .flush().unwrap();
    let y : f64 = input().unwrap();

    println! ("{} {} {} = {}",x,
                            match op{
                                Operation::Plus => "+",
                                Operation::Minus => "-",
                                Operation::Multiple => "*",
                                Operation::Divide => "/" 
                            },
                            y,
                            Operation::calc(x, y, op));
}


fn main() {
    calculatexy();
}
