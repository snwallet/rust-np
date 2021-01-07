use mylib::factory::produce_refrigerator as A;

use mylib::factory::*;
fn main() {

    mylib::factory::produce_refrigerator::produce_re(); //绝对路径

    //推荐
    produce_refrigerator::produce_re(); //use

    //A::produce_re();
    println!("Hello, world!");
}
