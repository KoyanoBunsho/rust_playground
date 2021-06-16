fn main() {
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
    change(&mut s1);
    {
        let r1 = &mut s1;
    }
    let r2 = &mut s1;
    let a1 = &s1;
    let a2 = &s1;
    println!("{} {}", a1, a2);
    let a3 = &mut s1;
    println!("{}", a3);
    let reference_to_nothing = dangle();
}

fn dangle() -> String {
    let s = String::from("hello");
    s
}

fn change(some_string: &mut String) {
    some_string.push_str(", world.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
