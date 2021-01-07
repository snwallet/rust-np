use std::collections::HashMap;

fn main() {
    let mut scores : HashMap<String,i32> = HashMap::new();
    scores.insert(String::from("blue"),10);
    scores.insert(String::from("red"),30);

    println!("{:#?}",scores);

    let k = String::from("blue");
    if let Some(v) = scores.get(&k){
        println!("{}",v);
    }

    //遍历 无序
    for (key,value) in &scores {
        println!("{},{}",key,value);
    }

    //插入
    let mut ss = HashMap::new();
    ss.insert(String::from("1"),1);
    //ss.insert(String::from("2"),2);
    ss.insert(String::from("3"),3);
    println!("{:#?}",ss);

    ss.entry(String::from("2")).or_insert(345);
    println!("{:#?}",ss);



    let text = "hello world wonderful world";
    let mut map  = HashMap::new();
    for word in text.split_whitespace()  {
        let count = map.entry(word).or_insert(0);
        *count+=1;
    }
    println!("{:#?}",map);
}
