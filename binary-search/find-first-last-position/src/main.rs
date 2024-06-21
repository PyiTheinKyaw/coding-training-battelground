use std::collections::HashMap;

fn main() {
    // let nums = vec![5,7,7,8,8,10];
    // let nums = vec![2,2];
    // let nums = vec![1];
    // let nums = vec![1,1,2];
    // let nums = vec![1,3];
    let nums = vec![3,3,3];

    let target = 3;

    let result = search_range(nums, target);

    println!("Result: {:?}", result);
}

pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    
    use std::cmp::Ordering;

    let mut left = 0;
    let mut right = nums.len();

    let mut result: Vec<i32> = vec![];

    while left < right {

        let mid = left + (right - left) / 2;

        match nums[mid].cmp(&target) {
            Ordering::Equal => {
                let mut is_previous_added = false;
                let mut is_next_value_added = false;
                let _cache_len = result.len();

                if mid != 0 && nums.get(mid-1).is_some()
                    && nums[mid] == nums[mid-1] {
                    result.push((mid-1) as i32);
                    is_previous_added = true;
                }

                if nums.get(mid+1).is_some() 
                    && nums[mid] == nums[mid+1] {

                    if is_previous_added {
                        result.push((mid+1) as i32);
                    } else {
                        result.push(mid as i32);
                        result.push((mid+1) as i32);
                    }
                    
                    is_next_value_added = true;
                }

                if (is_previous_added && !is_next_value_added) 
                    || (_cache_len == result.len()) {
                    result.push((mid) as i32);
                }
            
                break;
            },
            Ordering::Greater => right = mid,
            Ordering::Less => left = mid + 1,
        }
    }

    result = if result.len() == 0 { vec![-1, -1] } else { result };
    result = if result.len() == 1 { vec![result[0], result[0]] } else { result };

    result
}