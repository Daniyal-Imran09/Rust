use std::io;

fn main() {
    println!("Enter the first string:");
    let mut string1 = String::new();
    io::stdin().read_line(&mut string1).expect("Failed to read line");

    println!("Enter the second string:");
    let mut string2 = String::new();
    io::stdin().read_line(&mut string2).expect("Failed to read line");

    let string1 = string1.trim();
    let string2 = string2.trim();

    let concatenate_string = concatenate_strings(&string1, &string2);
    println!("{}", concatenate_string);
}

fn concatenate_strings(s1: &str, s2: &str) -> String {
    let mut result = String::from("");
    result.push_str(s1);
    result.push_str(s2);

    result
}
