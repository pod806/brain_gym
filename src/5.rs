use std::collections::VecDeque;
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

    while i < ifade.len() {
        let karakter = ifade.chars().nth(i).unwrap();

        if karakter == ' ' {
            i += 1;
            continue;
        }

        if karakter.is_digit(10) {
            let mut deger = 0.0;
            while i < ifade.len() && ifade.chars().nth(i).unwrap().is_digit(10) {
                deger = deger * 10.0 + ifade.chars().nth(i).unwrap().to_digit(10).unwrap() as f32;
                i += 1;
            }

            if i < ifade.len() && ifade.chars().nth(i).unwrap() == '.' {
                i += 1;
                let mut kesir = 0.1;
                while i < ifade.len() && ifade.chars().nth(i).unwrap().is_digit(10) {
                    deger += ifade.chars().nth(i).unwrap().to_digit(10).unwrap() as f32 * kesir;
                    kesir *= 0.1;
                    i += 1;
                }
            }

            degerler.push_back(deger);
            println!("Sayı eklendi: {}", deger);
            durum_yazdir(&degerler, &islemler);
            continue;
        }


        if karakter == '(' {
            islemler.push_back(karakter);
            println!("Açma parantezi eklendi");
            durum_yazdir(&degerler, &islemler);
        }

        else if karakter == ')' {
            while !islemler.is_empty() && *islemler.back().unwrap() != '(' {
                let val2 = degerler.pop_back().unwrap();
                let val1 = degerler.pop_back().unwrap();
                let op = islemler.pop_back().unwrap();
                let sonuc = islem_uygula(val1, val2, op);
                degerler.push_back(sonuc);
                println!("Parantez içi işlem: {} {} {} = {}", val1, op, val2, sonuc);
                durum_yazdir(&degerler, &islemler);
            }
            islemler.pop_back(); // Açma parantezini kaldır
            println!("Kapanma parantezi işleme alındı");
            durum_yazdir(&degerler, &islemler);
        }

        else {
            while !islemler.is_empty() && oncelik(*islemler.back().unwrap()) >= oncelik(karakter) {
                let val2 = degerler.pop_back().unwrap();
                let val1 = degerler.pop_back().unwrap();
                let op = islemler.pop_back().unwrap();
                let sonuc = islem_uygula(val1, val2, op);
                degerler.push_back(sonuc);
                println!("İşlem: {} {} {} = {}", val1, op, val2, sonuc);
                durum_yazdir(&degerler, &islemler);
            }
            islemler.push_back(karakter);
            println!("Operatör eklendi: {}", karakter);
            durum_yazdir(&degerler, &islemler);
        }

        i += 1;
    }
    while !islemler.is_empty() {
        let val2 = degerler.pop_back().unwrap();
        let val1 = degerler.pop_back().unwrap();
        let op = islemler.pop_back().unwrap();
        let sonuc = islem_uygula(val1, val2, op);
        degerler.push_back(sonuc);
        println!("Kalan işlem: {} {} {} = {}", val1, op, val2, sonuc);
        durum_yazdir(&degerler, &islemler);
    }

    degerler.pop_back().unwrap()
}

fn main() {
    println!("Sonuç: {}\n", ifade_degerlendir("2*3+5/6*3+15"));

}
