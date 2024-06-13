### Steps and Mathematical Equations

1. **Initialize Variables**:
   - \( \text{numbers} = [1, 2, \ldots, n] \) (list of numbers to permute)
   - Adjust \( k \) to be zero-based: \( k \leftarrow k - 1 \)
   - \( \text{permutation} \) (an empty list to store the resulting permutation)

2. **Determine the Position of Each Digit**:
   - For each position in the permutation (from left to right), we determine which number to place by using factorials to find the size of each block of permutations starting with the same first digit.

3. **Iterate Through Positions**:
   - For \( i \) from \( n \) down to 1:
     1. **Calculate Factorial**:
        \[
        \text{factorial} = (i-1)!
        \]
     2. **Determine Index**:
        \[
        \text{index} = \left\lfloor \frac{k}{\text{factorial}} \right\rfloor
        \]
     3. **Update \( k \)**:
        \[
        k = k \mod \text{factorial}
        \]
     4. **Select Number**:
        - Add the number at the calculated `index` from the `numbers` list to the `permutation` list.
        - Remove this number from the `numbers` list.

4. **Convert the Result to a String**:
   - Join the numbers in the `permutation` list to form the final permutation string.

### Example Walkthrough for \( n = 3 \) and \( k = 3 \):

1. **Initialization**:
   - \( \text{numbers} = [1, 2, 3] \)
   - \( k = 3 - 1 = 2 \)
   - \( \text{permutation} = [] \)

2. **First Digit** (\( i = 3 \)):
   - Calculate \( (3-1)! = 2! = 2 \)
   - Determine `index`: \( \left\lfloor \frac{2}{2} \right\rfloor = 1 \)
   - Update \( k \): \( 2 \mod 2 = 0 \)
   - Select number at index 1: 2
   - Update \( \text{numbers} = [1, 3] \)
   - Update \( \text{permutation} = [2] \)

3. **Second Digit** (\( i = 2 \)):
   - Calculate \( (2-1)! = 1! = 1 \)
   - Determine `index`: \( \left\lfloor \frac{0}{1} \right\rfloor = 0 \)
   - Update \( k \): \( 0 \mod 1 = 0 \)
   - Select number at index 0: 1
   - Update \( \text{numbers} = [3] \)
   - Update \( \text{permutation} = [2, 1] \)

4. **Third Digit** (\( i = 1 \)):
   - Only one number left in `numbers`: 3
   - Select number: 3
   - Update \( \text{numbers} = [] \)
   - Update \( \text{permutation} = [2, 1, 3] \)

5. **Result**:
   - Join the `permutation` list to form the final string: "213"

This systematic approach leverages the properties of factorials to efficiently determine the \( k \)-th permutation without generating all permutations.