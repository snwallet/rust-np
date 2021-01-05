//1、rust通过所有权机制来管理内存，编译器在编译就会根据所有权规则对内存的使用进行检查
//2、堆和栈
    //编译的时候数据的类型大下是固定的,就是分配在栈上的
    //编译的时候数据类型大小不固定，就是分配在堆上的
//3、作用域

//4、String内存回收
//5、移动
//6、clone
//7、栈上数据拷贝
//8、函数和作用域
fn main() {
    let x:i32 = 1;//分配在栈上
    let y:i32 = 1;
    println!("{}{}",x,y);

    {
        let mut s1 = String::from("hello");//不知道大小 分配在堆上
        s1.push_str("jsdfskdfjsdf");
        println!("{}",s1); //String类型离开作用域的时候会调用drop方法释放内存

        let s2 = String::from("hello");//不知道大小 分配在堆上
        let s3 = s2;
        println!("{}",s3);
        //println!("{}",s2);//move到s3  s2无效

        //克隆 clone
        let s4 = s3.clone();
        println!("{}",s4);

    }

    //copy
    //可以copy的类型
    //所有整型、浮点型、布尔型、字符类型、元组
    let a= 1;
    let b =a;
    println!("{}    {}",a,b);

    println!("Hello, world!");
}
