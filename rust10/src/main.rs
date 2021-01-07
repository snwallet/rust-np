//1、创建空的vector：vec<T>
//2、创建包含初始值的vector
//3、丢弃vector
//4、读取元素
//5、遍历
//6、使用枚举
fn main() {

    //1、创建空的vector：vec<T>
    let mut v:Vec<i32> = Vec::new();
    v.push(1);

    //2、创建包含初始值的vector
    let v = vec![1,2,3];

    //3、丢弃vector
    {
        let v1 = vec![1,2,3];
    }

    //4、读取元素
    let v = vec![1,2,3];
    let one = &v[1];
    println!("{}",one);

    //推荐
    match v.get(6){
        Some(value)=>println!("{}",value),
        _=>println!("sjdf"),
    }

    //5、更新

    let mut v2:Vec<i32> = Vec::new();
    v2.push(1);
    v2.push(23);
    v2.push(34543);


    //6、遍历
      //不可变的遍历
    for i in &v2 {
        println!("{}",i);
    }
      //可变的遍历
    for i in &mut v2 {
        *i+=1;
        println!("{}",i);
    }
    //7、使用枚举
    enum c{
        Text(String),
        Float(f32),
    }
    let a = vec![
        c::Text(String::from("343242sdfdg")),
        c::Float(234.234),
    ];



    println!("Hello, world!");
}
