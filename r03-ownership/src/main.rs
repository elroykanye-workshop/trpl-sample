fn ex1_strings() {
    let mut s : String = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
}

fn ex2_moving() {
    let x = 5;
    let y = x.clone();

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{}", s2);
    println!("{}", s1);


}

fn ex3_1_ownership() {
    let s = String::from("hello");
    ex3_2_takes_ownership(s);
    // println!("{}", s); // compile time error

    let x = 5;
    ex3_3_makes_copy(x);
    println!("{}", x + 9);
}

fn ex3_2_takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string is out of scope and drop is called.
// The backing memory is freed

fn ex3_3_makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // some_integer goes out of scope. Nothing special happens

fn ex4_1_returns() {
    let s1 = ex4_2_gives_ownership();

    let s2 = String::from("hello");
    let s3 = ex4_3_takes_and_gives_back(s2);

}

fn ex4_2_gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn ex4_3_takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn ex5_1_returns() {
    let s1 = String::from("hello");
    let (s2, len) = ex5_2_calculate_length(s1);
    println!("The length of '{}' is {}", s2, len);
}

fn ex5_2_calculate_length(some_string: String) -> (String, usize) {
    let len = some_string.len();
    (some_string, len)
}

fn ex6_1_ref_borrow() {
    let s1 = String::from("hello");
    let len = ex6_2_calculate_length(&s1);
    println!("The length of '{}' if {}", s1, len);
}

fn ex6_2_calculate_length(some_string: &String) -> usize {
    some_string.len()
}

fn ex7_1_ref_borrow() {
    let mut s = String::from("hello");
    ex7_2_change(&mut s);
}

fn ex7_2_change(some_string: &mut String) {
    some_string.push_str(", world!");
}







// Main function bellow
fn main() {
    ex6_1_ref_borrow();
}
