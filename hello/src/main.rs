pub fn hello()-> &'static str{
    "Hello World"
}
fn main(){
    println!("{}",hello());
    return;
}