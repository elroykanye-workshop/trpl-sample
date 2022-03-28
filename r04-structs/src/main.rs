struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Point3D(i32, i32, i32);

// Rectangle Struct
#[derive(Debug)]
struct Rectangle {
    width: u32, height: u32,
}

// methods
impl Rectangle {
    fn area (&self) -> u32{
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// associated functions
impl Rectangle {
    fn make_square(size: u32) -> Rectangle {
        Rectangle {width: size, height: size}
    }
}
// Rectangle Struct end

fn ex01 () {
    let elroy: User = User {
        username: String::from("elroy"),
        email: String::from("elroy@gmail.com"),
        sign_in_count: 190,
        active: false,
    };

    println!("{}", elroy.sign_in_count);
}

fn ex02_build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 0
    }
}

fn ex03() {
    let elroy: User = User {
        username: String::from("elroy"),
        email: String::from("elroy@gmail.com"),
        sign_in_count: 190,
        active: false,
    };

    let kanye: User = User {
        username: String::from("kanye"),
        email: String::from("kanye@gmail.com"),
        ..elroy
    };
}

fn ex04() {
    let point1 = Point3D(3, 4, 5);
}

fn ex05_1() {
    let width1 = 30;
    let height1 = 50;

    println!("The area of the rectangle is {} square pixels",
        ex05_2_area(width1, height1)
    );
}

fn ex05_2_area(width: u32, height: u32) -> u32{ width * height }

fn ex06_1() {
    let rect1: (u32, u32) = (30, 50);

    println!("The area of the rectangle is {} square pixels",
        ex06_2_area(rect1)
    );
}

fn ex06_2_area(dimensions: (u32, u32)) -> u32 { dimensions.0 * dimensions.1}

fn ex07_1() {
    let rect1: Rectangle = Rectangle { width: 30, height: 50 };
    println!("The area of the rectangle is {} square pixels",
        ex07_2_area(&rect1)
    );
}

fn ex07_2_area(rect: &Rectangle) -> u32 { rect.width * rect.height }

fn ex08() {
    let rect1 = Rectangle {width: 30, height: 50};
    println!("rect1 is {:#?}", rect1);
}

fn ex09() {
    let rect1 = Rectangle {width: 30, height: 50 };

    println!("The area of the rectangle is {} square pixels",
             rect1.area()
    );
}

fn ex10() {
    let sq1 = Rectangle::make_square(4);
    println!("The area of the rectangle is {} square pixels",
             sq1.area()
    );
}


fn main() {
    ex10();
}
