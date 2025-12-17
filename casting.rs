use std::io;

/*
tipos primitivos de variáveis

bool - Boolean (true / false)

char - character

f32, f64 - 32-bits, 64-bits floats

i64, i32, i16, i8 - signed 16- ... integers
u64, u32, u16, u8 - unsigned 16-bits, ... integers

isize - pointer-sized signed integers
usize - pointer-sized unsigned integers
*/

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    // let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");
}