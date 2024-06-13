pub fn get_permutation(n: i32, k: i32) -> String {
    
    let mut n_list: Vec<u64> = (1..=n.abs() as u64).collect();
    let mut permutation_list: Vec<u64> = vec![];
    let mut k_value = k - 1;

    for i in (0..n_list.len()).rev() {
        
        let factorial_value = factorail(i);
        let index = (k_value / factorial_value as i32) as usize;


        k_value = k_value % factorial_value as i32;

        permutation_list.push(n_list[index]);
        n_list.remove(index);
    }
    
    permutation_list
        .iter()
        .map(|&x|  char::from_digit(x as u32, 10).unwrap())
        .collect()
}

pub fn factorail(n: usize) -> usize {
    if n == 0 || n == 1 {
        return 1;
    } else {
        return n * factorail(n - 1);
    }
}


fn main() {
    let k = 16;
    let n = 5;
    let result = get_permutation(n, k);

    println!("k: {:?}", k);
    println!("n: {:?}", n);
    println!("kth value: {:?}", result);
}