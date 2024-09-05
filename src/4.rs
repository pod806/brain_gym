use std::io;

fn palindrome(num: i32) -> bool {
    if num < 0 {
        return false; 
    }

    let num_str = num.to_string();
    let reversed: String = num_str.chars().rev().collect();
    num_str == reversed
}

fn main() {
    println!("Bir sayı girin:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Girdi okunamadı");
    
    let num: i32 = input.trim().parse().expect("Geçerli bir sayı gir");

    if palindrome(num) {
        println!("{} palindrom sayıdır.", num);
    } else {
        println!("{} palindrom sayı değildir.", num);
    }
}
