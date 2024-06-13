To solve the problem of finding the number of different arrays consisting of numbers from $1$ to $n$ that have exactly $k$ inverse pairs, we use a dynamic programming approach.

### Step-by-Step Explanation

1. **Definitions and Base Case**:
    - Let $dp[n][k]$ represent the number of arrays with $n$ elements that have exactly $k$ inverse pairs.
    - Base case: $dp[0][0] = 1$. An empty array has exactly 0 inverse pairs.
2. **Transition**:
    - When we add the number $i$ (from $1$ to $n$) to an array of length $n-1$, we need to consider all possible positions where $i$ can be inserted.
    - Inserting $i$ at position $j$ (where $0 \leq j \leq n-1$) will create $j$ inverse pairs, because $i$ will be greater than all the elements to its left.
    
    Therefore, the recurrence relation is:
    $$
    dp[n][k] = \sum_{j=0}^{\min(k, n-1)} dp[n-1][k-j]
    $$
    
3. **Optimizing with Sliding Window**:
    - To optimize the calculation, we use a sliding window approach to compute the sum efficiently.

### Detailed Math Example

Let's compute $dp[3][2]$ (number of arrays with 3 elements having exactly 2 inverse pairs):

1. Initialize the DP array with base case:
$$dp[0][0] = 1$$

2. Fill the DP table:
    
    For $n=1$:
$$dp[1][0] = 1$$

For $n=2$:
$$dp[2][0] = 1 \quad \text{[1, 2]}$$
    
$$dp[2][1] = 1 \quad \text{[2, 1]}$$

    For $n=3$:
$$dp[3][0] = 1 \quad \text{[1, 2, 3]}$$

$$
dp[3][1] = dp[2][1] + dp[2][0] = 1 + 1 = 2 \quad \text{[2, 1, 3], [1, 3, 2]}
$$
$$
dp[3][2] = dp[2][2] + dp[2][1] + dp[2][0] = 0 + 1 + 1 = 2 \quad \text{[3, 1, 2], [2, 3, 1]}
$$
    

### Efficient Pseudo Code

Here is the optimized pseudo code for calculating the number of arrays with exactly $k$ inverse pairs:

```
function kInversePairs(n, k)
    if k == 0 then return 1
    if k > n * (n-1) / 2 then return 0

    dp = array of size (n+1) x (k+1) initialized to 0
    dp[0][0] = 1

    for i from 1 to n
        for j from 0 to k
            if j == 0
                dp[i][j] = dp[i-1][j]
            else
                dp[i][j] = dp[i][j-1] + dp[i-1][j]
                if j >= i
                    dp[i][j] = dp[i][j] - dp[i-1][j-i]

    return dp[n][k]

```

### Explanation of the Pseudo Code

1. **Initialization**:
    - We initialize a 2D array `dp` with dimensions $(n+1) \times (k+1)$ to store the number of arrays for each $n$ and $k$.
2. **Base Case**:
    - $dp[0][0] = 1$ since an empty array has exactly 0 inverse pairs.
3. **Filling the DP Table**:
    - For each $i$ from 1 to $n$:
        - For each $j$ from 0 to $k$:
            - If $j == 0$, then $dp[i][j] = dp[i-1][j]$ because there are no inverse pairs.
            - Otherwise, use the sliding window technique to sum up the values efficiently.
4. **Return the Result**:
    - The value $dp[n][k]$ gives the number of arrays with $n$ elements that have exactly $k$ inverse pairs.

This dynamic programming approach ensures efficient computation with a time complexity of $O(n \times k)$.