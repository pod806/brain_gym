use std::collections::VecDeque;
use std::io::{self, Write};

fn oncelik(op: char) -> i32 {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => 0,
    }
}

fn islem_uygula(a: f32, b: f32, op: char) -> f32 {
    match op {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => a / b,
        _ => 0.0,
    }
}

fn durum_yazdir(degerler: &VecDeque<f32>, islemler: &VecDeque<char>) {
    println!("Degerler: {:?}", degerler);
    println!("Islemler: {:?}", islemler);
}

fn ifade_degerlendir(ifade: &str) -> f32 {
    let mut degerler: VecDeque<f32> = VecDeque::new();
    let mut islemler: VecDeque<char> = VecDeque::new();
    let mut i = 0;
    let mut number_buffer = String::new();

    while i < ifade.len() {
        let karakter = ifade.chars().nth(i).unwrap();

        if karakter == ' ' {
            i += 1;
            continue;
        }

        if karakter.is_digit(10) || karakter == '.' {
            number_buffer.push(karakter);
        } else {
            if !number_buffer.is_empty() {
                degerler.push_back(number_buffer.parse::<f32>().unwrap());
                number_buffer.clear();
            }

            match karakter {
                '(' => islemler.push_back(karakter),
                ')' => {
                    while let Some(op) = islemler.pop_back() {
                        if op == '(' {
                            break;
                        }
                        let val2 = degerler.pop_back().unwrap();
                        let val1 = degerler.pop_back().unwrap();
                        let sonuc = islem_uygula(val1, val2, op);
                        degerler.push_back(sonuc);
                    }
                }
                '+' | '-' | '*' | '/' => {
                    while let Some(&last_op) = islemler.back() {
                        if oncelik(last_op) >= oncelik(karakter) {
                            let val2 = degerler.pop_back().unwrap();
                            let val1 = degerler.pop_back().unwrap();
                            let sonuc = islem_uygula(val1, val2, last_op);
                            degerler.push_back(sonuc);
                            islemler.pop_back();
                        } else {
                            break;
                        }
                    }
                    islemler.push_back(karakter);
                }
                _ => {}
            }
        }
        i += 1;
    }

    if !number_buffer.is_empty() {
        degerler.push_back(number_buffer.parse::<f32>().unwrap());
    }

    while let Some(op) = islemler.pop_back() {
        let val2 = degerler.pop_back().unwrap();
        let val1 = degerler.pop_back().unwrap();
        let sonuc = islem_uygula(val1, val2, op);
        degerler.push_back(sonuc);
    }

    degerler.pop_back().unwrap()
}

fn main() {
    let mut input = String::new();
    print!("Lütfen matematiksel ifadeyi girin: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim(); // boşlukları temizle

    let sonuc = ifade_degerlendir(input);
    println!("Sonuç: {}", sonuc);
}
