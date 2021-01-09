//struct a{
//    w:u32,
//    h:u32,
//}
//
//fn mianji(e:&a)->u32{
//    e.w*e.h
//}
//fn main(){
//    let rect = a{
//        w:32,
//        h:43,
//    };
//    println!("{}",mianji(&rect));
//}

#[derive(Debug)]
struct A{
    w:u32,
    h:u32,
}

impl A {

    fn area(&self) ->u32{
        self.h*self.w
    }
    fn can_hold(&self, other: &A) -> bool {
        self.w > other.w && self.h > other.h
    }
}
fn main(){
    let s = A{
        w:32,
        h:1,
    };
    let s1= A{
        w:45,
        h:23,
    };
    println!("{}",s.area());
    println!("{}",s.can_hold(&s1));

}