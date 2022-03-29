use std::fmt::Display;

pub fn ex01() {
    {
        let r;

        let x = 5;
        r = &x;

        println!("r: {}", r);
    }
}

pub fn ex02_1() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = ex02_2_longest(string1.as_str(), string2);
    println!("Longest = {}", result);
}

fn ex02_2_longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

fn ex03_longest_with_announcement<'a, T>(str1: &'a str, str2: &'a str, ann: T)
    -> &'a str where T: Display {
    println!("Announcement! {}", ann);
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}