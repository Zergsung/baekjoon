use std::io;
// 입출력과 사칙연산 10번

fn decompose_number(num1: i32) -> (i32, i32, i32) {
    let num1_thd = num1 % 10;
    let num1_snd = (num1 % 100) / 10;
    let num1_fst = num1 / 100;

    (num1_thd, num1_snd, num1_fst)
}

fn main() {
    let mut num1 = String::new();
    let mut num2 = String::new();

    io::stdin()
        .read_line(&mut num1)
        .expect("Failed to read input");
    let mut num1:i32 = match num1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Plz int");
            return;
        }
    };

    io::stdin()
        .read_line(&mut num2)
        .expect("Failed to read input");
    let mut num2:i32 = match num2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Plz int");
            return;
        }
    };
    let (num2_fst, num2_snd, num2_thd) = decompose_number(num2);

    println!("{}\n{}\n{}\n{}", num1 * num2_fst, num1 *  num2_snd, num1 * num2_thd, num1 * num2);
}
