const MAX_POINT:u32 = 10000;
fn main(){
    //1、变量定义
    //定义变量用let 如果变量没有用mut，那么是不可变的
    //let name : type
    let a = 1;
    let mut b:u32 = 2;
    println!("{},{}",a,b);

    b=3;
    println!("{}",b);

    //隐藏
    let b:f32 = 1.9;
    println!("{}",b);

    //常量
    println!("MAX_POINTS={}",MAX_POINT);
    println!("Hello, world!");

    //bool
    let is_true = true;
    let is_false = false;

    println!("{},{}",is_false,is_true);


    //char 在rust里面char是32位的 可以是一个汉字（c、c++ char 8位 ）
    let a = 'a';
    let b = '弄';
    println!("{},{}",a,b);


    //数字类型 i8 i16 i32 i64  u8 u32 u64  f32 f64
    let c =-111;
    let d = 0.0001;
    println!("{},{}",c,d);

    //自适应类型
    //isize usize 无符号

    println!("{}",isize::max_value());


    //数组[Type ; size] size也是数组类型的一部分

    let arr3 :[u32;5] = [1,2,3,4,5];
    let arr1 :[u32;3] = [2,2,3];
    println!("{}",arr3[1]);

    show(arr1);

    //元组
    let tup = (-3,3.98,'和');
    println!("{},{},{}",tup.0,tup.1,tup.2);

    let (x,y,z) = tup;

    println!("{},{},{}",x,y,z);
}
fn show(arr:[u32;3]){
    for i in &arr {
        println!("{}",i);
    }
}
