fn main() {
    let char_vec: Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];
    let iter = char_vec.iter().map(|c| char::from_u32(*c as u32 + 1).unwrap());
    let res: Vec<char> = iter.collect();
    for elements in res {
        println!("{}", elements);
    }

}
