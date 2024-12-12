
fn get_input_type(c: char) -> i32 {
    if c.is_alphabetic() {
        1
    } else {
        0
    }
}

fn main() {
    let str = "Don't ask what your country can do for you, but ask what you can do for your country.";
    let mut state = 0; // init state
    let mut input = 0; // 0: space		1: alpha
    let mut counter = 0;

    println!("hello, count how many words in string:");
    println!("<{}>", str);

    for c in str.chars() {
        input = get_input_type(c);

        if state == 0 && input == 1 {
            state = 1;
        } else if state == 1 && input == 0 {
            counter += 1;
            state = 0;
        } 
    }

    println!("find {} words.", counter);
}
