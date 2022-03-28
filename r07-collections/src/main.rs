use std::collections::HashMap;

fn ex01() {
    let mut v1: Vec<i32> = Vec::new();
    v1.push(5);
    v1.push(6);
    v1.push(7);
    v1.push(8);

    let third = v1.get(20);
    println!("{}", third.is_some())
}

fn ex02() {
    let mut v1: Vec<i32> = vec![1, 2, 3, 4, 5,];
    for i in &mut v1 {
        *i += 50;
    }
    println!("{:?}", v1);
}

fn ex03() {
    // STRINGS
    let data = "initial contents";
    let s = data.to_string();
    let s1 = "initial contents".to_string();
    let s2 = String::from("initial contents");
}

fn ex04() {
    // HASH MAPS
    let mut scores1 = HashMap::new();
    scores1.insert(String::from("Blue"), 10);
    scores1.insert(String::from("Yellow"), 50);

    let mut teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores2: HashMap<&String, &i32> = teams.iter().zip(initial_scores.iter()).collect();
}

fn ex05() {
    // HASH MAP Ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);

    let s = field_name;
}

fn main() {
    ex05();
}
