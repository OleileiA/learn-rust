use std::io; // 引入IO标准库

fn main () {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new(); // mut 可变的变量声明
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}