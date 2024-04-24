#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn greating(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub fn this_fn_cant_handle_input_greater_than_100(number: u8) -> bool {
    if number < 100 {
        return true;
    }
    panic!("see!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle::new(10, 20);
        let smaller = Rectangle::new(8, 9);

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle::new(10, 20);
        let smaller = Rectangle::new(8, 9);

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn add_to_4() {
        assert_ne!(add(2, 2), 5);
    }

    #[test]
    fn greating_carol() {
        let result = greating("carol");
        assert!(
            result.contains("carol"),
            "result did not contain name, the value was {}",
            result
        )
    }

    #[test]
    #[should_panic(expected = "see")]
    fn this_should_panic() {
        this_fn_cant_handle_input_greater_than_100(101);
    }
}
