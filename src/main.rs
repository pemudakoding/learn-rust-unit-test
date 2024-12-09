fn main() {
    println!("Hello, world!");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn start_application(host: &str, port: u16) {
    if host == "localhost" {
        panic!("You cannot use localhost as host!");
    }

    println!("host {} is starting at {}", host, port);
}


#[cfg(test)]
mod tests {
    use crate::{add, start_application};

    #[test]
    fn test_simple() {
        println!("Hello, world!");
    }
    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3, "1 + 2 should be 3");
    }
    #[test]
    #[should_panic]
    fn test_start_application() {
        start_application("localhost", 1234);
    }

    #[test]
    #[ignore]
    fn test_ignored() {
        println!("Hello, world!");
    }

    #[test]
    fn test_add_again() -> Result<(), String> {
        let result = add(1, 2);

        if result == 3 {
            Ok(())
        } else {
            Err(String::from("I don't understand what went wrong!"))
        }
    }
}