fn main() {
    let x_str = "abcdefgh";
    let y_str = "abcdefg";
    println!("is x > y ? : {}", compareString(x_str, y_str));
}
fn compareString(x : &str, y : &str) -> bool {
    let chars_x : Vec<char> = x.chars().collect();
    let chars_y : Vec<char> = y.chars().collect();
    let mut index = 0;
    let mut flag = false;
    loop {
        if index >= chars_x.len() || index >= chars_y.len() {
            if(chars_x.len() > chars_y.len()) {
                flag = true;
            }
            break;
        }
        if chars_x[index] > chars_y[index] {
            flag = true;
            break;
        }
        index += 1;
    }
    flag
}
