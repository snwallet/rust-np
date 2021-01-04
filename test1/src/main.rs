use std::io::prelude::*;
use std::fs::File;

fn main(){
    //let text = fs::read_to_string("D:\\123.txt").unwrap();
    //let text = fs::read("D:\\123.txt").unwrap();
    let mut file = File::create("D:\\123.txt").unwrap();
    file.write(b"hjsrgfjskdhf").unwrap();
    //println!("{:?}",text);
}