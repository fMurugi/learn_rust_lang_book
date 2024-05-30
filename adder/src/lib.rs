pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn add_two(x:i32) -> i32 {
    x + 2
}

fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}
#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle{
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

struct Guess{
    value:i32
}

fn guess(value:i32) -> Guess {
    if value < 1 || value > 100 {
        panic!("Guess value must be between 1 and 100, got {}", value);
    }
    Guess {
        value
    }
}

pub fn add_num(x:i32) -> i32 {
    internal_adder(x, 2)
}
fn internal_adder(a:i32, b:i32) -> i32 {
    a + b
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller(){
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger(){
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(!smaller.can_hold(&larger));
    
    }

    #[test]
    fn it_adds_two(){
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name(){
        let result = greeting("Carol");
        assert!(result.contains("Carol"), "Greeting did not contain name, value was '{}'", result);
        print!("result: {}", result);
    }

    #[test]
    #[should_panic="Guess value must be between 1 and 100, got 200"]
    fn greater_than_100(){
        guess(200);
    }

    #[test]
    #[ignore]
    fn expensive_test()  {
        // code that takes an hour to run
        
    }

    #[test]
    fn it_adds(){
        assert_eq!(4, add(2, 2));
    }

}

