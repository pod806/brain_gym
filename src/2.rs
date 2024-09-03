use std::io;                                                           // 2. soru dizi azaltma

/*
fn reduce_size(arr: &mut Vec<i32>, reduce_by: usize) {  //2. soru input almadan
    let new_length = arr.len().saturating_sub(reduce_by);

    arr.truncate(new_length);
}

fn main() {
    let mut arr = vec![1, 3, 15, 11, 2];
    let reduce_by = 2;

    reduce_size(&mut arr, reduce_by);

    for i in 0..arr.len() {
        print!("{} ", arr[i]);
    }
}
*/

fn main() {
    let mut input = String::new();
    println!("Dizi elemanlarını girin:");
    io::stdin().read_line(&mut input).expect("Satır okunamadı");

    let elements: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Dönüşüm hatası"))
        .collect();

    let mut input = String::new();
    println!("Çıkartılacak eleman sayısı:");
    io::stdin().read_line(&mut input).expect("Satır okunamadı");
    let num_to_remove: usize = input.trim().parse().expect("Dönüşüm hatası");

    let num_to_remove = num_to_remove.min(elements.len());

    let reduced_elements: Vec<i32> = elements
        .iter()
        .take(elements.len() - num_to_remove)
        .copied()
        .collect();

    println!("Azaltılmış dizi: {:?}", reduced_elements);
}