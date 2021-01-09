//use std::ops::Mul;
//
//fn largest <T> (list : &[T]) -> T
//where T:std::ops::Mul<Output = T> + Copy
//{
//    let mut i = 0;
//    for &item in list.iter() {
//        i +=1;
//    }
//    println!("{}",i);
//    if i>1 {
//        println!(">1");
//        let mut largest0 = list[0];
//        let mut largest1 = list[1];
//        return largest0 * largest1;
//    }else {
//        let mut largest0 = list[0];
//       return largest0* largest0* 3;
//    }
//
//}
//
//fn main() {
//    let number_list = vec![3, 5];
//
//    let result = largest(&number_list);
//    println!("The largest number is {}", result);
//
//    let number_list = vec![3];
//
//    let result = largest(&number_list);
//    println!("The largest number is {}", result);
//}

use std::ops::Mul;

struct Rect<T>
{
    width: T,
    height: T
}

impl<T: Mul<Output = T> + Copy> Rect<T> {
    fn area(&self) -> T{
        self.height * self.width
    }
}

struct R<T> {
    r: T,
}

impl<T: Mul<Output = T> + Copy> R<T> {
    fn area2(&self) -> T{
        let s = self.r * self.r *3.14;
        s
    }
}

fn main() {
    // 整型
    let rect1 = Rect{width:3, height:4};
    println!("{}", rect1.area());

    // 浮点型
    let rect2 = Rect{width:3.5, height:4.3};
    println!("{}", rect2.area());

    let rect3 = R{r:3};
    println!("{}", rect3.area2());
}