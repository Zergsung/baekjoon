use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let mut nums_iter = input.split_whitespace(); // 입력받은 문자열을 split_whitespace() 메소드를 이용하여 공백으로 분리된 문자열 슬라이스들로 분할합니다.
    
    let num1: i32 = nums_iter.next().unwrap().parse().unwrap();
    let num2: i32 = nums_iter.next().unwrap().parse().unwrap();
    // next() 메소드를 두번 호출하여, 숫자로 변환된 두 개의 값이 변수 num1과 num2에 저장됩니다. 마지막으로 println! 매크로를 이용하여 결과를 출력합니다.

    println!("{}", num1 - num2);
}
