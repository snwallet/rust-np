//1、泛型是具体类型或者其他属性的抽象替代，用于减少代码重复
//2、在函数定义中使用泛型
//3、在结构体中使用泛型
//4、枚举中的泛型
//5、方法中的泛型
//6、使用泛型并不会造成程序性能上的损失，rust 通过在编译时进行泛型代码的单态化来保证效率。
// 单态化时通过填充编译时使用的具体类型，将通过代码转化为特定代码的过程

fn largest_i32(list:&[i32])->i32{
    let mut largest = list[0];
    for &item in list.iter() {
        if item >largest{
            largest = item;
        }
    }
    largest
}

//2、在函数定义中使用泛型
fn largest <T : PartialOrd + Copy>(list : &[T]) -> T{
    let mut larger = list[0];
    for &item in list.iter() {
        if item > larger{
            larger = item;
        }
    }
    larger
}

//fn main() {
//
//    let number_list = vec![1,100,34,2354,234,42222222];
//    let number_list1 = vec!['a','e','y','t'];
//    //let max_number = largest_i32(&number_list);
//
//    let max_number = largest(&number_list1);
//    println!("{}",max_number);
//    //println!("Hello, world!");
//}

//在结构体
//#[derive(Debug)]
//struct Point<T>{
//    x:T,
//    y:T,
//}
//#[derive(Debug)]
//struct Point2<T,U>{
//    x:T,
//    y:U,
//}

//fn main(){
//    let integer = Point2{x:2.3,y:4};
//    println!("{:#?}",integer)
//}

//在枚举中
enum Option<T>{
    Some(T),
    None,
}

enum Result<T,E>{
    Ok(T),
    Err(E)
}

//在方法中使用
//#[derive(Debug)]
struct Point<T>{
    x:T,
    y:T,
}

impl <T> Point<T>{
    fn get_x(&self) ->&T{
        &self.x
    }
    fn get_y(&self) ->&T{
        &self.y
    }
}


struct Point2<T,U>{
    x:T,
    y:U,
}

impl <T,U> Point2<T,U>{
    fn mixup<V,W>(self,other:Point2<V,W>) ->Point2<T,W>{
        Point2{
            x:self.x,
            y:other.y,
        }
    }

}
fn main(){
//    let i = Point3{x:2,y:4};
//    println!("{}",i.get_x());

    let p = Point2{x:1.1,y:23};
    let p2 = Point2{x:"jsdf",y:'e'};

    let p3 = p.mixup(p2);
    println!("{},{}",p3.x,p3.y);
}