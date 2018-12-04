use std::io::Read;
use std::fs::File;
use std::collections::HashMap;


fn count_twos_and_threes(value:&str, first:&mut bool)->(i32, i32)
{
    let mut twos:i32 = 0;
    let mut threes:i32 = 0;
    let mut h:HashMap<char, i32> = HashMap::new();
    if *first {
        println!("str={}", value);
    }
    for c in value.chars() {
        if h.contains_key(&c) {
            let tmp = h.get(&c).unwrap() + 1;
            h.insert(c, tmp);
        } else {
            h.insert(c, 1);
        }
    }
    for tmp in h.keys() {
        let val = h.get(tmp).unwrap();
        if *first {
            println!("Char {} count {}", *tmp, *val);
        }
        if *val==2 && twos == 0{
            twos = twos + 1;
        }
        if *val==3 && threes == 0 {
            threes = threes + 1;
        }
    }
    *first = false;
    (twos, threes)
}

fn part1() 
{
    let mut f = File::open("input.txt").unwrap();
    let mut contents = String::new();
    let l = f.read_to_string(& mut contents).unwrap();
    drop(f);
    println!("{} bytes l = {}", contents.len(), l);
    let mut iter = contents.split_whitespace();
    let mut lines = 0;
    let mut first = false;
    let mut twoct = 0;
    let mut threect = 0;
    loop {
        let v = iter.next();
        match v {
            Some(value) => {
                lines = lines + 1;
                let (two, three) = count_twos_and_threes(&value, & mut first);
                twoct += two;
                threect += three;
            }
            None => {break;}
        }
    }
    println!("Code = {} x {} = {}", twoct, threect, twoct*threect);
}

fn compare(s1:&str, s2:&str, debug:bool) -> (i32, i32)
{
    let mut mmct:i32 = 0;
    let mut mmpos:i32 = 0;
    let c1 :Vec<char> = s1.chars().collect();
    let c2 :Vec<char> = s2.chars().collect();
    for i in 0..s1.len() {
        if  c1[i] != c2[i] {
            mmct = mmct + 1;
            mmpos = i as i32;
            if debug {
                println!("c1={} c2={} pos = {}", c1[i], c2[i], mmpos);
            }
        } else {
            if debug {
                println!("c={} pos={}", c1[i], i);
            }
        }
    }
    (mmct, mmpos)
}

fn part2()
{
    let mut f = File::open("input.txt").unwrap();
    let mut contents = String::new();
    let _ = f.read_to_string(& mut contents).unwrap();
    drop(f);
    let bc_list:Vec<&str> = contents.split_whitespace().map(|x| x.trim()).collect();
    for i in 0..bc_list.len() {
        for j in i+1..bc_list.len() {
            let (ct, pos) = compare(bc_list[i], bc_list[j], false);
            if ct < 2 {
                println!("[{}] [{}] ct = {} pos {}", bc_list[i], bc_list[j], ct, pos);
                let s0 :String = bc_list[i].chars().take(pos as usize).collect();
                let s1 :String = bc_list[i].chars().skip((pos+1) as usize).collect();
                let s2 :String = bc_list[j].chars().take(pos as usize).collect();
                let s3 :String = bc_list[j].chars().skip((pos+1) as usize).collect();
                println!("{}{}", s0, s1);
                println!("{}{}", s2, s3);
            }
        }
    }
}

fn main() {

    part2();
}
