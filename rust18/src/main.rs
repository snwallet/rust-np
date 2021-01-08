//1、trait_bound语法
//2、指定多个trait bound
//3、返回trait的类型

trait  GetName{
    fn get_name(&self) ->&String;
}

trait  GetAge{
    fn get_age(&self) -> u32;
}

fn print_infomation<T:GetName+GetAge>(item:T){
    println!("name={}",item.get_name());
    println!("age={}",item.get_age());
}

//实现trait
#[derive(Debug)]
pub struct Student{
    pub name:String,
    pub age:u32,
}

impl GetName for Student{
    fn get_name(&self) ->&String{
        &self.name
    }
}

impl GetAge for Student{
    fn get_age(&self) ->u32{
        self.age
    }
}

fn produce_item_with_age() -> impl GetAge{
    Student{
        name:String::from("xiaoming"),
        age:15,
    }
}

#[derive(Debug)]
pub struct Teacher{
    pub name:String,
    pub age:u32,
}

impl GetName for Teacher{
    fn get_name(&self) ->&String{
        &self.name
    }
}

impl GetAge for Teacher{
    fn get_age(&self) ->u32{
        self.age
    }
}
//错误
//fn produce_item_with_age2() -> impl GetAge{
//    let is = true;
//    if is {
//        Student{
//            name:String::from("xiaoming"),
//            age:15,
//        }
//    }else {
//        Teacher{
//            name:String::from("hhsdbfbdsf"),
//            age:345,
//        }
//    }
//}

fn main() {
    let s= Student{
        name:"xiaoming".to_string(),
        age:10,
    };

    let s = produce_item_with_age();
    //println!(":#?",s);
    //print_infomation(s);
    println!("Hello, world!");
}
