fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
    let x = 5;
    let y = x;
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{}, world", s1);
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
    let sr = String::from("hello");
    takes_ownership(sr);
    let x = 5;
    makes_copy(x);
    let a1 = gives_ownership();
    let a2 = String::from("hello");
    let a3 = takes_and_gives_back(a2);
    let (a4, len) = calculate_length(a3);
    println!("The length of {} is {}.", a4, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
