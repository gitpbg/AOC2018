extern crate regex;
use regex::Regex;

#[derive(Debug)]
struct Rectangle {
    id:String,
    x:i32,
    y:i32,
    w:i32,
    h:i32
}

impl Rectangle {
    fn new() -> Rectangle {
        Rectangle {
            id: String::from("foo"),
            x:0,
            y:0,
            w:0,
            h:0
        }
    }
}

fn parse_string(re:&Regex, data:&str) -> Option<Rectangle>
{
    let mut rv = Rectangle::new();
    let caps = re.captures(&data).unwrap();
    rv.id = String::from(caps.name(&"id").unwrap().as_str());
    rv.x = caps.name(&"xpos").unwrap().as_str().parse::<i32>().unwrap();
    rv.y = caps.name(&"ypos").unwrap().as_str().parse::<i32>().unwrap();
    rv.w = caps.name(&"width").unwrap().as_str().parse::<i32>().unwrap();
    rv.h = caps.name(&"height").unwrap().as_str().parse::<i32>().unwrap();
    Some(rv)
}

fn main() {
    //read_data()
    let re = Regex::new(r"(?P<id>#\d+) @ (?P<xpos>\d+),(?P<ypos>\d+): (?P<width>\d+)x(?P<height>\d+)").unwrap();
    let s = String::from("#1 @ 509,796: 18x15");
    let rv = parse_string(&re, &s);
    println!("{}=>{:?}", s, rv);
}
