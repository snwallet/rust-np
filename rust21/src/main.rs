//1、rust中每一个引用都有其生命周期，也就是引用保持有效的作用域。大部分生命周期是隐含并可以推断的，正如大部分的时候类型可以推断一样
//2、生命周期的主要目标是避免悬垂引用
//3、rust编译器使用借用检查器来检查生命周期是否有效

//fn main() {
//    let r;
//    {
//        let x = 5;
//        r = &x;
//        println!("{}",r);
//    }
//
//
//}

//函数中的生命周期
//fn longest (x:&str,y:&str) -> &str{
fn longest<'a> (x:&'a str,y:&'a str) -> &'a str{
    if x.len()>y.len(){
        x
    }else {
        y
    }
}

fn longest22<'a> (x:&'a str,y:&str) -> &'a str{
    x
}
fn main() {
    let r = longest22("234","sdghfsdbv");
    println!("{}",r);
}
