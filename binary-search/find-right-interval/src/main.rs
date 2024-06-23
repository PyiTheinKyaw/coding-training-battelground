fn main() {
    let intervals = vec![vec![1,4],vec![2,3],vec![3,4]];
    let result = find_right_interval(intervals);

    println!("Result: {:?}", result);
}


pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {

    let mut indexed_intervals: Vec<(i32, i32)> = intervals
        .iter()
        .enumerate()
        .map(|(i, x)| (x[0], i as i32))
        .collect();


    indexed_intervals.sort_by_key(|&(start, _)| start);

    let mut result = vec![-1; intervals.len()];

    for (i, interval) in intervals.iter().enumerate() {
        let end = interval[1];

        let mut left = 0;
        let mut right = indexed_intervals.len();

        while left < right {

            let mid = left + (right - left) / 2;

            if indexed_intervals[mid].0 >= end {
                right = mid;
            }
            else {
                left = mid + 1;
            }
        }

        if left < indexed_intervals.len() { 
            result[i] = indexed_intervals[left].1;
        }
    }
    
    result
}