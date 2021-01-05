//引用 & 引用之后不会销毁（dorp）
//创建一个指向值的运用，但不拥有它，不拥有这个值，所以引用离开其值指向的作用域后也不会丢弃
//借用：&mut
fn calcute_length(s:&String) ->usize{
    s.len()
}
fn modify_s(s:&mut String){
    s.push_str(",sdjgfsdh");
}
fn dangle() ->  String{
    let s = String::from("hsh");
    &s
}
fn main() {
//    let mut s1 = String::from("sjhsdf");
//    modify_s(&mut s1);
//
//    println!("{}",s1);//还可以用

    let s = dangle();
    println!("{}",s);
}


//fn main() {
//    //1
////    let arr:[i32;9] = [1,2,3,4,5,6,7,8,9];
////    for i in &arr{
////        for j in &arr {
////            if i>=j {
////                print!("{}*{}={}\t", i, j, i * j);
////            }
////        }
////        println!("");
////    }
//
//// 2
////    let mut i = 0;
////    let mut j = 0;
////    while i<9 {
////        i+=1;
////        while j<9 {
////            j+=1;
////            if i>=j {
////                print!("{}*{}={}\t", i, j, i * j);
////            }
////            if j ==9{
////                j = 0;
////                break;
////            }
////        }
////        println!();
////    }
//
//
//    let mut i = 0;
//    let mut j = 0;
//    loop {
//        i += 1;
//        loop {
//            j += 1;
//            if i >= j {
//                print!("{}*{}={}\t", i, j, i * j);
//            }
//            if j == 9 {
//                j = 0;
//                break;
//            }
//        }
//        if i == 9 {
//            break;
//        }
//        println!();
//    }
//}

