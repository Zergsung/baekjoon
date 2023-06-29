use std::io;
// 입출력과 사칙연산 11번

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let mut nums_iter = input.split_whitespace(); // 입력받은 문자열을 split_whitespace() 메소드를 이용하여 공백으로 분리된 문자열 슬라이스들로 분할합니다.
    
    let num1: i64 = nums_iter.next().unwrap().parse().unwrap();
    let num2: i64 = nums_iter.next().unwrap().parse().unwrap();
    let num3: i64 = nums_iter.next().unwrap().parse().unwrap();

    // let result = num1 + num2 + num3;

    println!("{}", num1 + num2 + num3);
}
