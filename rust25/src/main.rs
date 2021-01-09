use std::ops::Mul;
//use std::f32::consts::PI;
use std::io;

struct Rect<T, U>{
    width: T,
    height: U,
}

impl<T: Mul<Output = T> + Copy,U: Into<T> + Copy> Rect<T, U> {
    fn area(&self) -> T{
        self.width.mul(self.height.into())
    }
}

struct R<T> {
    r: T,
}

impl<f32: Mul<Output = f32> + Copy> R<f32> {
    fn area2(&self) -> f32{
        self.r * self.r
    }
}

fn main() {
    let mut i =0;
    loop {
        i +=1;
        if i<3 {
            println!("第{}次输入:",i);
        }
        if i==3 {
            let rect2 = Rect{width:3.1, height:45.3};
            println!("矩形的面积为{}", rect2.area());
            break;
        }
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let trimmed = guess.trim();
        if trimmed  != "0" {
        }else {
            if i==2 {
                let rect3 = R{r:3};
                println!("圆的面积为{}", rect3.area2());
            }
            break;
        }

    }
}