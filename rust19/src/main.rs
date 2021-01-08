//使用trait bound 有条件的实现方法
trait GetName{
    fn get_name(&self) ->&String;
}
trait GetAge{
    fn get_age(&self) ->u32;
}
struct PeopleMetchInformation<T,U>{
    master:T,
    stduent:U,
}
impl <T:GetAge+GetName,U:GetAge+GetName>PeopleMetchInformation<T,U>{
    fn print_all_information(&self){
        println!("{}",self.master.get_age());
        println!("{}",self.master.get_name());
        println!("{}",self.stduent.get_age());
        println!("{}",self.stduent.get_name());
    }
}

struct Teacher{
    name:String,
    age:u32,
}
impl GetName for Teacher{
    fn get_name(&self) ->&String{
        &(self.name)
    }
}

impl GetAge for Teacher{
    fn get_age(&self) ->u32{
        self.age
    }
}


struct Student{
    name:String,
    age:u32,
}
impl GetName for Student{
    fn get_name(&self) ->&String{
        &(self.name)
    }
}

impl GetAge for Student{
    fn get_age(&self) ->u32{
        self.age
    }
}
fn main() {
    let s = Student{
        name:"xiaoming".to_string(),
        age:12,
    };
    let t = Teacher{
        name:"jsdhbfdf".to_string(),
        age:33,
    };

    let m = PeopleMetchInformation{
        master:t,
        stduent:s,
    };
    m.print_all_information();
    println!("Hello, world!");
}
