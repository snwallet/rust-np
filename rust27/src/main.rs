use serde::{Deserialize, Serialize};
use serde_json::{Result};

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    age: u8,
    xuehao: String,
    addr: String,
}

fn main() -> Result<()> {

    let json = r#"
    {
        "name": "张翰",
        "age": 23,
        "xuehao": "12323",
        "addr": "shdjfs"
     }"#;

//    let v: Value = serde_json::from_str(json)?;
//
//    println!("name = {}", v["name"]);
//    println!("age = {}", v["age"]);
//    println!("xuehao = {}", v["xuehao"]);
//    println!("addr = {}", v["addr"]);


    let u: User = serde_json::from_str(json)?;

    println!("name = {}", u.name);
    println!("age = {}", u.age);
    println!("xuehao = {}", u.xuehao);
    println!("addr = {}", u.addr);

    Ok(())
}