# Explanation of K Inverse Pairs Array

An inverse pair in an array is a pair of indices $(i, j)$ such that $i < j$ and $A[i] > A[j]$. For example, in the array $[3, 1, 2]$, the inverse pairs are $(3, 1)$ and $(3, 2)$.

Given two integers $n$ and $k$, the task is to find the number of different arrays consisting of numbers from $1$ to $n$ that have exactly $k$ inverse pairs.

## Approach

To solve this problem, we use dynamic programming. Let's define $dp[n][k]$ as the number of arrays with $n$ elements having exactly $k$ inverse pairs.

### Recurrence Relation

1. **Base Case**:
   - $dp[0][0] = 1$: An empty array has exactly 0 inverse pairs.

2. **Transition**:
   - When we add the number $i$ (from $1$ to $n$) to an array of length $n-1$, we need to consider all possible positions where $i$ can be inserted. Inserting $i$ at position $j$ will create $j$ inverse pairs (since $i$ will be greater than all the elements to its left).
   - Therefore, the recurrence relation is:
     $$
     dp[n][k] = \sum_{j=0}^{\min(k, n-1)} dp[n-1][k-j]
     $$
   - This sum can be computed efficiently using a sliding window sum to avoid recomputation.

## Pseudo Code

Here's a pseudo code for the dynamic programming solution:

```pseudo
function kInversePairs(n, k)
    if k == 0 then return 1
    if k > n * (n-1) / 2 then return 0

    dp = array of size (n+1) x (k+1) initialized to 0
    dp[0][0] = 1

    for i from 1 to n
        for j from 0 to k
            for m from 0 to min(j, i-1)
                dp[i][j] = dp[i][j] + dp[i-1][j-m]

    return dp[n][k]
```

## Optimized Pseudo Code

To optimize the above pseudo code by using a sliding window sum:

```pseudo
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

### Explanation of Optimized Pseudo Code

1. **Base Case**:
   - We start with $dp[0][0] = 1$.

2. **Transition**:
   - For each $i$ from 1 to $n$:
     - For each $j$ from 0 to $k$:
       - Use a sliding window sum to accumulate the values.
       - $dp[i][j] = dp[i][j-1] + dp[i-1][j]$: Adding the previous value in the row and the value directly above.
       - If $j \geq i$: Subtract the value that is out of the window to maintain the correct sum.

This optimized version ensures that we are not recalculating the sum from scratch each time, making it more efficient.

## Complexity

- **Time Complexity**: $O(n \times k)$
- **Space Complexity**: $O(n \times k)$

This dynamic programming approach allows us to efficiently compute the number of arrays with exactly $k$ inverse pairs.