extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;




use std::fs::File;
#[derive(Debug, Serialize, Deserialize)]
struct Address {
    street: String,
    city: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    address: Address,
    phones: Vec<String>,
}
//fn main() {
//
//    let f = File::open("./src/main.json").unwrap();
//    let v: Person = serde_json::from_reader(f).unwrap();
//    println!("{:#?}", v);
//}
use serde_json::Value;
fn main() {
    let f = File::open("./src/main.json").unwrap();
    let v: serde_json::Value = serde_json::from_reader(f).unwrap();
    println!("{:?}", v["name"].as_str().unwrap());
    //println!("{:?}", v["phones"].as_str().unwrap());
    println!("{:?}", v["age"].as_i64().unwrap());


    let mut v = json!({"a":"somthing","b":10});
    v.as_object_mut().unwrap().insert("c".to_string(), serde_json::Value::String("new value".to_string()));
}

//#[macro_use]
//extern crate serde_json;
//
//use serde_json::Value;
//
//fn merge(a: &mut Value, b: Value) {
//    match (a, b) {
//        (a @ &mut Value::Object(_), Value::Object(b)) => {
//            let a = a.as_object_mut().unwrap();
//            for (k, v) in b {
//                merge(a.entry(k).or_insert(Value::Null), v);
//            }
//        }
//        (a, b) => *a = b,
//    }
//}
//
//fn main() {
//    let mut a = json!({
//        "title": "This is a title",
//        "person" : {
//            "firstName" : "John",
//            "lastName" : "Doe"
//        },
//        "cities":[ "london", "paris" ]
//    });
//
//    let b = json!({
//        "title": "This is another title",
//        "person" : {
//            "firstName" : "Jane"
//        },
//        "cities":[ "colombo" ]
//    });
//
//    merge(&mut a, b);
//    println!("{:#}", a);
//}
