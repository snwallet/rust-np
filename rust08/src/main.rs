enum IpAddrKind{
    V4,
    V6,
}

struct Ipaddr{
    kind:IpAddrKind,
    address:String,
}

//rust语言提倡的方式
enum IpAddr2{
    V4(String),
    V6(String),
}



//不同类型
enum IpAddr3{
    V4(u8,u8,u8,u8),
    V6(String),
}
//经典用法
enum Message{
    Quit,
    Move{x:i32,y:i32},
    Write(String),
    Change(i32,i32,i32),
}

//枚举类型的方法以及match
impl Message{
    fn prin(&self){
        match *self {
            Message::Quit =>println!("quit"),
            Message::Move {x,y} =>println!("{},{}",x,y),
            Message::Change (a,b,c) =>println!("{},{},{}",a,b,c),
            _=>println!("write"),
            //Message::Write(&s) =>println!("write"),
        }
    }
}
fn main() {
    let i1 = Ipaddr{
        kind:IpAddrKind::V4,
        address:String::from("127.0.0.1"),
    };
    let i2 = Ipaddr{
        kind:IpAddrKind::V6,
        address:String::from("::1"),
    };

    let i1 = IpAddr2::V4(String::from("127.0.0.1"));
    let i2 = IpAddr2::V6(String::from("::1"));

    let i1 = IpAddr3::V4(127,0,0,1);
    let i2 = IpAddr3::V6(String::from("::1"));

    println!("Hello, world!");

    let quit = Message::Move {x:2,y:21};
    quit.prin();

    let quit = Message::Write(String::from("sdfgsdf"));
    quit.prin();


}

