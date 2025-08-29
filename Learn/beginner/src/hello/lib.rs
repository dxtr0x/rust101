pub fn hello()-> &'static str{
    "Hello World"
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ishello() {
        assert_eq!(hello(), "Hello World");
    }
}