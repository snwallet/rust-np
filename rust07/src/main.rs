#[derive(Debug)]
struct Dog{
    name:String,
    weight:f32,
    height:f32,
}

impl Dog{
    fn get_name(&self)->&str{
        &(self.name[..])
    }
    fn get_weight(&self) ->f32{
        self.weight
    }
    fn get_height(&self) ->f32{
        self.height
    }
    fn show(){
        println!("wwawa");
    }
}
fn main() {
    let dog = Dog{
        name:String::from("wndsfg"),
        weight:189893.0,
        height:2340.4,
    };
    println!("{:#?}",dog);
    Dog::show();
    println!("Hello, world!");
}
