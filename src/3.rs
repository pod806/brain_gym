fn faktoriyel(n:i32)->i32{                                                              //3
    let mut sonuc= 1;
    for i in 2..=n {
        sonuc = sonuc*i;
    }
    sonuc
}

fn permutasyon(n:i32 , r:i32)->i32{
    faktoriyel(n)/faktoriyel(n-r)
}

fn main() {
    let n=10;
    let r=3;
    let sonuc=permutasyon(n,r);
    println!("permutasyon sonucu {}", sonuc);
}