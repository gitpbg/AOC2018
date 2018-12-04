struct Rectangle {
    id:String,
    x:i32,
    y:i32,
    w:i32,
    h:i32
}

impl Rectangle {
    fn from_string(s:&str) -> Rectangle {
        Rectangle {
            id: String::from("foo"),
            x:0,
            y:0,
            w:0,
            h:0
        }
    }
}

fn main() {
    //read_data()
    let s = String::from("#1 @ 509,796: 18x15");
    let m = s.matches("[0-9]+");
    for a in m {
        println!("{:?}", a);
    }
}
