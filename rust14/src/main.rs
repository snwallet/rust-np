use std::io;
use std::io::Read;
use std::fs::File;

//fn read_username_from_file() ->Result<String,io:: Error>{
//    let f = File::open("123.txt");
//    let mut f= match f{
//        Ok(file) =>file,
//        Err(error) =>return Err(error),
//    };
//
//    let mut s= String::new();
//    match f.read_to_string(&mut s){
//        Ok(_)=>Ok(s),
//        Err(error) => Err(error),
//    }
//}

fn read_username_from_file() ->Result<String,io:: Error>{
    let mut s= String::new();
    let f = File::open("123.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
fn main() {
   let r =  read_username_from_file();
    match r {
        Ok(s)=>println!("{}",s),
        Err(e) => println!("{:#?}",e),
    }
    println!("Hello, world!");
}
