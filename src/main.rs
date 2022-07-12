use std::io; // prelude 导入 std 库中的 io 模块

fn main() {
    println!("Input a number");

    let mut expected_number = String::new();

    io::stdin().read_line(&mut expected_number).expect("can't read");

    println!("your answer is {}", expected_number);
}