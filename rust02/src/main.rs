fn other_fun(){
    println!("你好");
}
fn other_fun01(a:u32,b:i32){
    println!("{},{}",a,b);
}
fn other_fun02(a:i32,b:i32)->i32{
    let result = a+b;
    return result;
}

fn other_fun03(a:i32,b:i32)->i32{
//    let result = a+b;
//    result
    a*b
}
fn main() {
    other_fun();
    other_fun01(2,-23);
    let r = other_fun02(2,-15);
    let e = other_fun03(2,-15);
    println!("{}",e);

    //语句是执行一些操作，但不返回值的指令
    //let x = (let y = 3)错误
    //表达式会计算值
    let y= {
        let x = 1;
        x + 5 //没有分号
    };
    println!("{}",y);
    println!("Hello, world!");

    //循环
    let y = 222;
    if y==1 {
        println!("y=1");
    }
    else   {
        println!("y!=1");
    }


    if y==1 {
        println!("y=1");
    } else if y==0  {
        println!("y!=1");
    }else {
        println!("hhh");
    }

    let condition = false;
    let x = if condition{
        5
    }else {
        6
    };
    println!("{}",x);



    //loop
    let mut counter = 0;
    loop{
        println!("loop");
        if counter ==10{
            break ;
        }
        counter +=1;
    }

    let result = loop{
        counter +=1;
        if counter ==20 {
            break counter;
        }
    };
    println!("{}",result);


    //while
    let mut i = 1;
    while i !=10 {
        i += 1;
    }
    println!("{}",i);

    //for

    let arr:[i32;5] = [1,2,3,4,5];
    //for i in arr.iter() {
    for i in &arr {
        print!("{}",i)
    }
}
