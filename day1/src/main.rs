use std::io::Read;
use std::fs::File;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    let len = file.read_to_string(& mut contents).unwrap();
    println!("{} bytes read", len);
    let mut iter = contents.split_whitespace();
    let mut count = 0;
    loop { 
        let v = iter.next();
        match v {
            Some(value) => { count+=value.parse::<i32>().unwrap();}
            None => break
        }
    }
    println!("number is {}", count);
}
