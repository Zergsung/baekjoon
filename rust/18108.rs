use std::io;
// 입출력과 사칙연산 8번
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let input: i32 = input.trim().parse().unwrap_or(0);
    println!("{}", input - (2541 - 1998));
}
