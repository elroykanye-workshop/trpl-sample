
struct Point<T> {
    x: T, y: T
}

impl <T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}


pub fn ex01_1_generics() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = ex01_2_largest(&number_list);
    println!("The largest number is {}", largest)
}

fn ex01_2_largest(list: &[i32]) -> i32 {
    let mut largest = 0;
    for number in list {
        if *number > largest { largest = *number; }
    }
    largest
}

/*
pub fn ex02_1_generics() {
    let num_list = vec![34, 50, 25, 100, 65];

    let res = ex02_2_largest(&num_list);
    println!("The largest number is {}", res);

    let char_list = vec!['e', 'l', 'r', 'o', 'y'];
    let res = ex02_2_largest(&char_list);
    println!("The largest character is {}", res);
}

fn ex02_2_largest<T>(list: &[T]) -> T {
    let mut largest = &list[0];

    for &item in list.iter() {
        if item > largest { largest = item; }
    }
    largest
}

 */

pub fn ex03() {
    let int_point = Point{x: 5, y: 10};
    let float_point = Point {x: 1.0, y: 3.4};
    println!("float_point = [{} , {}]", float_point.x(), float_point.y())
}

