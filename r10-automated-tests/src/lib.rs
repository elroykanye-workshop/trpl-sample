#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn does_not_work() {
        //panic!("Make this test fail!")
    }

    #[test]
    fn can_hold_rect_1_test() {
        let larger = Rectangle { length: 8, width: 7, };
        let smaller = Rectangle { length: 5, width: 1, };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn can_hold_rect_2_test() {
        let larger = Rectangle { length: 8, width: 7, };
        let smaller = Rectangle { length: 5, width: 1, };

        assert!(!smaller.can_hold(&larger));
    }
}

#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}
impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}