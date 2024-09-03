use std::io;

fn en_kucuk_fark(dizi1: &[i32], dizi2: &[i32]) -> i32 {
    let mut sorted_dizi1 = dizi1.to_vec();
    let mut sorted_dizi2 = dizi2.to_vec();

    sorted_dizi1.sort();
    sorted_dizi2.sort();

    let mut i = 0;
    let mut j = 0;
    let mut en_kucuk_deger = i32::MAX;

    while i < sorted_dizi1.len() && j < sorted_dizi2.len() {
        let fark = (sorted_dizi1[i] - sorted_dizi2[j]).abs();

        if fark < en_kucuk_deger {
            en_kucuk_deger = fark;
        }
        if sorted_dizi1[i] < sorted_dizi2[j] {
            i += 1;
        } else {
            j += 1;
        }
    }

    en_kucuk_deger
}

fn main() {
    let mut input = String::new();

    println!("Dizi 1 elemanlarını boşluklarla ayırarak girin:");
    io::stdin().read_line(&mut input).expect("Satır okunamadı");
    let dizi1: Vec<i32> = input.trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Dönüşüm hatası"))
        .collect();

    input.clear();

    println!("Dizi 2 elemanlarını boşluklarla ayırarak girin:");
    io::stdin().read_line(&mut input).expect("Satır okunamadı");
    let dizi2: Vec<i32> = input.trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Dönüşüm hatası"))
        .collect();

    let sonuc = en_kucuk_fark(&dizi1, &dizi2);
    println!("İki dizi arasındaki en küçük fark : {}", sonuc);
}