extern crate regex;
use regex::Regex;
use std::fs;
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
    let tmp = re.captures(&data);
    if tmp.is_some() {
        let caps = re.captures(&data).unwrap();
        rv.id = String::from(caps.name(&"id").unwrap().as_str());
        rv.x = caps.name(&"xpos").unwrap().as_str().parse::<i32>().unwrap();
        rv.y = caps.name(&"ypos").unwrap().as_str().parse::<i32>().unwrap();
        rv.w = caps.name(&"width").unwrap().as_str().parse::<i32>().unwrap();
        rv.h = caps.name(&"height").unwrap().as_str().parse::<i32>().unwrap();
        Some(rv)
    } else {
        None
    }
}

fn main() {
    //read_data()
    let data = fs::read_to_string("input.txt").unwrap();
    println!("{} bytes read", data.len());
    let v = data.split("\n");
    let re = Regex::new(r"(?P<id>#\d+) @ (?P<xpos>\d+),(?P<ypos>\d+): (?P<width>\d+)x(?P<height>\d+)").unwrap();
    let mut rects : Vec<Rectangle> = Vec::new();
    for val in v {
        println!("{}", val);
        let s = String::from(val);
        let rv = parse_string(&re, &s);
//        println!("{}=>{:?}", s, rv);
        match rv {
            Some(r) => rects.push(r),
            None=>{},
        }
    }
    println!("{} rectangles", rects.len());
}
