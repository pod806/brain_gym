use std::io::{self, Write};         // O(n) karmaşıklığında yapabildim.

fn is_palindrome_number(number: i32) -> bool {
    let num_str = number.to_string();
    num_str.chars().eq(num_str.chars().rev())
}

fn main() {
    print!("Bir sayı girin: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Okuma hatası");

    if let Ok(number) = input.trim().parse::<i32>() {
        if is_palindrome_number(number) {
            println!("{} palindrom sayıdır.", number);
        } else {
            println!("{} palindrom sayı değildir.", number);
        }
    } else {
        println!("Geçerli bir sayı giriniz.");
    }
}
