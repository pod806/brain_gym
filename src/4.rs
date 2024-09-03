fn is_palindrome_number(number: i32) -> bool {                                            //4.
    if number < 0 {
        return false;
    }

    let mut original_number = number;
    let mut reversed_number = 0;

    while original_number != 0 {
        let remainder = original_number % 10;
        reversed_number = reversed_number * 10 + remainder;
        original_number /= 10;
    }

    if reversed_number == number {
        true
    } else {
        false
    }
}

fn main() {
    let number = 5353535;
    if is_palindrome_number(number) {
        println!("{} palindrom sayıdır.", number);
    } else {
        println!("{} palindrom sayı değildir.", number);
    }
}