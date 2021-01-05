fn takes_ownership(some_string:String)->String{
    println!("{}",some_string);
    some_string
}
fn makes_copy(i:i32){
    println!("{}",i);
}
fn main() {

    let s = String::from("hello");
    let s1 = takes_ownership(s);
    println!("{}",s);//不能使用 堆中已经回收
    println!("{}",s1); //传回来可以使用

    let x = 3;
    makes_copy(x);
    println!("{}",x);//可以使用 栈中copy
    println!("Hello, world!");
}
