#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
	self.length > other.length 
	    && self.width > other.width
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
	if value < 1 || value > 100 {
	    panic!("Guess value must be between 1 and 100, got {}.", value);
	}

	Guess {
	    value
	}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
	assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
	panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
	let larger = Rectangle { length: 8, width: 7 };
	let smaller = Rectangle { length: 5, width: 1 };

	assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
	let larger = Rectangle { length: 8, width: 7 };
	let smaller = Rectangle { length: 5, width: 1 };

	assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
	assert_eq!(4, add_two(2));
    }

    #[test]
    fn result_not_equal() {
	let larger = Rectangle { length: 8, width: 7 };
	let smaller = Rectangle { length: 5, width: 1 };

	// an implementation of `std::cmp::PartialEq` might be missing for `Rectangle`
	// assert_eq!(larger, smaller);
    }

    #[test]
    fn greeting_contains_name() {
	let result = greeting("Carol");
	assert!(
	    result.contains("Carol"),
	    "Greeting did not contain name, value was `{}`", result
	);
    }

    #[test]
    #[should_panic(expected = "foo")]
    fn greater_than_100() {
	Guess::new(200);
    }

    #[test]
    fn it_works_result_error() -> Result<(), String> {
	if 2 + 2 == 4 {
	    Ok(())
	} else {
	    Err(String::from("two plus two does not equal four"))
	}
    }

    #[test]
    #[ignore]
    fn it_is_ignored() {
	panic!("Should not run")
    }
}
