pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
    const MOD_VALUE: i32 = 1000000007;

    if k == 0 { return 1 } // return if it's base case
    if k > (n * (n-1))/ 2 { return 0 } // Make sure 'k' is never execed the size of n.
    if !(1 <= n && n <= 1000) { return 0 }
    if !(0 <= k && n <= 1000) { return 0 }

    // Init n+1 x k+1 size 2D array with zero.
    let mut dp: Vec<Vec<i32>> = 
        vec![vec![0; k.abs() as usize +1]; n.abs() as usize +1];

    // Base Case
    dp[0][0] = 1;

    for i in 1..=n.abs() as usize {
        for j in 0..=k.abs() as usize {
            if j == 0 { dp[i][j] = 1 } // Base Case
            else if i == 1 { dp[i][j] = 0 }
            else {
                dp[i][j] = 0;

                for m in 0..=std::cmp::min(j, i-1) {
                    dp[i][j] += dp[i-1][j-m];
                    dp[i][j] %= MOD_VALUE;
                }
            }
        }
    }
    
    dp[n as usize][k as usize]

}

pub fn k_inverse_pairs_v2_slide_window(n: i32, k: i32) -> i32 {
    const MOD_VALUE: i32 = 1000000007;

    let mut dp:Vec<i32> = vec![0; k.abs() as usize + 1];

    dp[0] = 1;

    for i in 1..=n.abs() as usize {
        let mut perfix_sum: Vec<i32> = vec![0];
        let mut total = 0;

        for j in 0..=k.abs() as usize {
            
            total += dp[j];
            total %= MOD_VALUE;

            perfix_sum.push(total);
        }

        println!("{:?}", perfix_sum);

        for j in 0..=k.abs() as usize {

            let upper_bound = std::cmp::max(0, j as i32 - i as i32 + 1);
            let lower_bound = std::cmp::min(j + 1, k.abs() as usize + 1);

            dp[j] = perfix_sum[lower_bound] - perfix_sum[upper_bound.abs() as usize];
        }
    }

    dp[k.abs() as usize]
}

fn main() {
    let n: i32 = 1000;
    let k: i32 = 1000;

    println!("n {:?}", n);
    println!("k {:?}", k);

    let result = k_inverse_pairs(n, k);

    println!("Inversed Pairs Count {:?}", result);
    println!("Inversed Pairs Count {:?}", k_inverse_pairs_v2_slide_window(n, k));
}

