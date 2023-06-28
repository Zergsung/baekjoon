use std::io;
// 입출력과 사칙연산 2번
fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let mut nums_iter = input.split_whitespace();

    let num1: i32 = nums_iter.next().unwrap().parse().unwrap();
    let num2: i32 = nums_iter.next().unwrap().parse().unwrap();

    println!("{}", num1 + num2);
}