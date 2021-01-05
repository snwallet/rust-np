fn main() {

    //定义结构体
    #[derive(Debug)]
    struct User{
        name: String,
        count:String,
        nonce:u64,
        active:bool,
    }
    //创建结构体实例
    let xiaoming = User{
        name: String::from("xiaoming"),
        count:String::from("23443435"),
        nonce:1000,
        active:true,
    };
    println!("{:?}",xiaoming);
    println!("{:#?}",xiaoming);

    //修改结构体
    let mut xiaohuang = User{
        name: String::from("xiaohuang"),
        count:String::from("23443435"),
        nonce:1000,
        active:true,
    };
    xiaohuang.nonce = 20000;

    //参数名和字段同名可简写方法
    let name = String::from("sjhgf");
    let count = String::from("23234");
    let nonce = 30384;
    let active = false;

    let user1 = User{
        name,
        count,
        nonce,
        active,
    };

    //从其他结构体创建实例
    let user2 = User{
        ..user1
    };
    println!("{}",user2.name);

    //元组结构体
    struct Point(i32,i32);
    let a = Point(10,20);
    let b = Point(30,29);
    println!("{},{}",a.0,b.1)
}
