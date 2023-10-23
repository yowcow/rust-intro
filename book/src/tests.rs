#[allow(dead_code)]
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Self {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}",
                value
            )
        }
        if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}",
                value
            )
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::Guess;

    #[test]
    #[should_panic]
    fn test_new_panics() {
        Guess::new(101);
    }

    #[test]
    #[should_panic(expected = "greater than or equal to 1, got 0")]
    fn test_new_on_less_than_lower_limit() {
        Guess::new(0);
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100, got 101")]
    fn test_new_on_greater_than_upper_limit() {
        Guess::new(101);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 should be 4"))
        }
    }
}
