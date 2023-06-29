use std::io;
// 입출력과 사칙연산 7번
fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let input = input.trim();
    
    println!("{}??!", input);
}
