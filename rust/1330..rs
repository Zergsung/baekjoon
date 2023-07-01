use std::io;
// 조건문 1번
fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let nums: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let result = if nums[0] == nums[1] {
        "=="
    } else if nums[0] < nums[1] {
        "<"
    } else {
        ">"
    };

    println!("{}", result);
}
