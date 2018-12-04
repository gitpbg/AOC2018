use std::io::Read;
use std::fs::File;
use std::collections::HashMap;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    let len = file.read_to_string(& mut contents).unwrap();
    let mut count_map : HashMap<i32, i32> = HashMap::new();
    println!("{} bytes read", len);
    let mut iter = contents.split_whitespace();
    let mut count = 0;
    let mut first = true;
    'outer: loop {
    loop { 
        let v = iter.next();

        match v {
            Some(value) => { 
                if count_map.contains_key(&count) {
                    if first {
                        println!("Count {} reached twice", count);
                        first = false;
                        break 'outer;
                    }
                } else {
                    //println!("Value = {} Adding {}", value, count);
                    count_map.insert(count, 1);
                }
                count+=value.parse::<i32>().unwrap();
            }
            None => break
        }
    }
        iter = contents.split_whitespace();
    }
    println!("number is {}", count);
}
