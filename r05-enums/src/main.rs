fn ex01() {
    enum IpAddressKind {
        V4,
        V6
    }

    struct IpAddress {
        kind: IpAddressKind,
        address: String,
    }

    let home = IpAddress {
        kind: IpAddressKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddress {
        kind: IpAddressKind::V6,
        address: String::from("::1"),
    };
}

fn ex02 () {
    enum IpAddress {
        V4(String),
        V6(String),
    }

    let home = IpAddress::V4(String::from("127.0.0.1"));
    let loopback = IpAddress::V6(String::from("::1"));
}

fn ex03() {
    enum IpAddress {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddress::V4(127, 0, 0, 1);
    let loopback = IpAddress::V6(String::from("::1"));
}

fn ex04() {
    enum Message {
        Quit,
        Move {x: i32, y: i32},
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {

        }
    }

    let m = Message::Write(String::from("hello"));

}

fn ex05() {
    let some_number = Some(5);
    let none_number: Option<i32> = None;
}

fn ex06() {
    enum Coin {
        Penny, Nickel, Dime, Quarter
    }

    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime=> 10,
            Coin::Quarter => 25,
        }
    }

    let c1 = Coin::Nickel;
    println!("coin1 = {}",
        value_in_cents(c1)
    )
}

fn ex07() {
    fn plus_one(val: Option<i32>) -> Option<i32> {
        match val {
            Some(i) => Some(i + 1),
            None => None
        }
    }

    println!("4 plus 1 is {:?}",
        plus_one(Some(4))
    );
    println!("None plus 1 is {:?}",
        plus_one(None)
    )
}

fn ex08() {
    fn num_to_word(num: u8) -> () {
        match num {
            0 => println!("zero"),
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            4 => println!("four"),
            5 => println!("five"),
            _ => println!("Larger than five"),
        }
    }

    num_to_word(4);
    num_to_word(9);
}

fn ex09() {
    fn num_3_to_word(num: Option<i32>) -> () {
        if let Some(3) = num {
            println!("{:?}", num);
        }
    }

    num_3_to_word(Some(3));
    num_3_to_word(Some(4));
    num_3_to_word(None);
}

fn main() {
    ex09();
}
