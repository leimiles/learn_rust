use rand::Rng;
use std::io; // prelude 导入 std 库中的 io 模块

fn main() {
    let expected_number = rand::thread_rng().gen_range(1..101);

    println!("the expected number is {}", expected_number);

    println!("Input a number");

    let mut input_number = String::new();

    io::stdin()
        .read_line(&mut input_number)
        .expect("can't read");

    println!("your answer is {}", input_number);
}
