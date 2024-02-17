fn main() {
   
    let string1: &str = "Hello, ";
    let string2: &str = "World!";
    let concatenate_string = concatenate_strings(&string1,&string2);
    println!("{}",concatenate_string);

}

fn concatenate_strings(s1: &str,s2: &str) -> String {

    let mut result = String::from("");
    result.push_str(s1);
    result.push_str(s2);

    result

}
