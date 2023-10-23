#[derive(Debug, PartialEq)]
pub enum ShirtColor {
    Red,
    Blue,
}

pub struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    pub fn new(shirts: Vec<ShirtColor>) -> Self {
        Self { shirts }
    }

    pub fn giveaway(&self, user_pref: Option<ShirtColor>) -> Option<ShirtColor> {
        match user_pref {
            None => self.most_stocked(),
            other => other,
        }
    }

    pub fn most_stocked(&self) -> Option<ShirtColor> {
        if self.shirts.len() == 0 {
            return None;
        }

        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            Some(ShirtColor::Red)
        } else {
            Some(ShirtColor::Blue)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn most_stocked_to_return_red() {
        let ivt = Inventory::new(vec![ShirtColor::Red, ShirtColor::Red, ShirtColor::Blue]);

        assert_eq!(ivt.most_stocked(), Some(ShirtColor::Red));
    }

    #[test]
    fn most_stocked_to_return_blue() {
        let ivt = Inventory::new(vec![ShirtColor::Red, ShirtColor::Blue]);

        assert_eq!(ivt.most_stocked(), Some(ShirtColor::Blue));
    }

    #[test]
    fn most_stocked_to_return_none() {
        let ivt = Inventory::new(vec![]);

        assert_eq!(ivt.most_stocked(), None);
    }

    #[test]
    fn giveaway_with_preference() {
        let user_pref = Some(ShirtColor::Red);
        let ivt = Inventory::new(vec![ShirtColor::Blue, ShirtColor::Blue]);

        assert_eq!(ivt.giveaway(user_pref), Some(ShirtColor::Red));
    }

    #[test]
    fn giveaway_with_no_preference() {
        let user_pref = None;
        let ivt = Inventory::new(vec![ShirtColor::Blue, ShirtColor::Blue]);

        assert_eq!(ivt.giveaway(user_pref), Some(ShirtColor::Blue));
    }
}

#[cfg(test)]
mod test_inference {
    #[test]
    fn add_one() {
        let add_one_v2 = |x: i32| -> i32 { x + 1 };
        let add_one_v3 = |x| x + 1;
        let add_one_v4 = |x| x + 1;

        assert_eq!(add_one_v2(1), 2);
        assert_eq!(add_one_v3(1), 2);
        assert_eq!(add_one_v4(1), 2);
    }
}

#[derive(Debug, PartialEq)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

#[cfg(test)]
mod reference_or_ownership_tests {
    use super::Rectangle;

    #[test]
    fn test_mutable() {
        let mut list = vec![1, 2, 3];

        assert_eq!(list, vec![1, 2, 3]);

        let mut borrows_mutably = || list.push(7);
        borrows_mutably();

        assert_eq!(list, vec![1, 2, 3, 7]);
    }

    #[test]
    fn test_sort() {
        let mut list = [
            Rectangle {
                width: 10,
                height: 1,
            },
            Rectangle {
                width: 3,
                height: 5,
            },
            Rectangle {
                width: 7,
                height: 12,
            },
        ];

        let mut num_sort_operations = 0;
        list.sort_by_key(|r| {
            num_sort_operations += 1;
            r.width
        });

        assert_eq!(
            list,
            [
                Rectangle {
                    width: 3,
                    height: 5,
                },
                Rectangle {
                    width: 7,
                    height: 12,
                },
                Rectangle {
                    width: 10,
                    height: 1,
                },
            ]
        );
        assert_eq!(num_sort_operations, 6);
    }
}
