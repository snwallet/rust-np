//Option 是一个标准库定义的一个枚举
//
//enum  Option<T>{
//    Some(T),
//    None,
//}

fn plus_one(x:Option<i32>) ->Option<i32>{
    match x {
        None =>None,
        Some(x)=>Some(x+1),
    }
}
fn main() {

    let some_number = Some(5);
    let some_string = Some(String::from("sdknfdg"));
    let absent_number :Option<i32> = None;
    let x = 5;
    let y:Option<i32> = Some(5);

    let mut temp = 0;
    match y {
        Some(i)=>{
           temp = i;
        }
        None =>{
            println!("None");
        }
    }
    let sum = x+temp;
    println!("{}",sum);


//    let result = plus_one(y);
//    match result {
//        Some(i)=>println!("{}",i),
//        None=>None,
//    }

    if let Some(value) = plus_one(y){
        println!("y = {}",value);
    }else {
        println!("none");
    }

    println!("Hello, world!");
}
