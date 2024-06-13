fn main() {
    let n: i32 = 10;
    let k: i32 = 3;

    let result = k_inverse_pairs(n, k);


    println!("Inversed Pairs Count {:?}", result);
}


pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
    if k == 0 { return 1 }
    if k > (n * (n-1))/ 2 {return 0}

    
}