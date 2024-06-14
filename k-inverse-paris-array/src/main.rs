pub fn k_inverse_pairs_v3(n: i32, k: i32) -> i32 {
    const MOD_VALUE: i32 = 1000000007;

    if k == 0 { return 1 } // return if it's base case
    if k > (n * (n-1))/ 2 { return 0 } // Make sure 'k' is never execed the size of n.
    if !(1 <= n && n <= 1000) { return 0 }
    if !(0 <= k && n <= 1000) { return 0 }

    let mut dp: Vec<Vec<i32>> = vec![vec![0; k.abs() as usize +1]; n.abs() as usize +1];
    dp[0][0] = 1;

    // Generating inverse pair depend on n-size.
    for i in 1..=n.abs() as usize {

        let mut prefix_sum: Vec<i32> = vec![0; k.abs() as usize + 1];
        prefix_sum[0] = dp[i - 1][0]; // Get base case before loop k.

        for j in  1..=k.abs() as usize {
            prefix_sum[j] = (prefix_sum[j - 1] + dp[i-1][j]) % MOD_VALUE;
        }


        for j in 0..=k.abs() as usize {
            if j == 0 {dp[i][j] = dp[i-1][j]}

            else {
                dp[i][j] = prefix_sum[j];
                if j >= i {
                    dp[i][j] = (dp[i][j] - prefix_sum[j - i] + MOD_VALUE) % MOD_VALUE;
                }
            }
        }
    }

    dp[n as usize][k as usize]

}

fn main() {
    let n: i32 = 1000;
    let k: i32 = 1000;

    println!("n {:?}", n);
    println!("k {:?}", k);

    let result = k_inverse_pairs_v3(n, k);

    println!("Inversed Pairs Count {:?}", result);
    // println!("Inversed Pairs Count {:?}", k_inverse_pairs_v2_slide_window(n, k));
}

pub fn k_inverse_pairs_Efficient_code(n: i32, k: i32) -> i32 {
    let n = n as usize;
    let k = k as usize;
    let mut dp:[[i32; 1001]; 1002] = [[0; 1001]; 1002];
    dp[0][0] = 1;
    let md = 1000000007i64;
    for i in 1..=n{
        let l = (i *(i-1)/2).min(k);
        let min = ( (k+n) as i32- (n*n) as i32).max(0) as usize;
        let mut val = 0i64;
        for j in min ..= l{
            val += dp[i-1][j] as i64;
            if j >=i{
                val -= dp[i-1][j-i] as i64;
            }
            dp[i][j] = (val % md) as i32;
        }
    }
    return dp[n][k];
}