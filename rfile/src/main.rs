use std::{fmt::write, fs::File, io::Read};
 
fn main() {
    let mut txt = String::new();
    // let mut file = File::create("foo.txt").unwrap();
    // for _ in 1..10 {
    //     file.write(b"Hello world\n").unwrap();
    // }

    let mut read_f = File::open("foo.txt").unwrap();
    let _ = read_f.read_to_string(&mut txt);

    print!("{}",txt);
}
