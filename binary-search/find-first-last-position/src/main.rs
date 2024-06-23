fn main() {
    // let nums = vec![5,7,7,8,8,10];
    // let nums = vec![2,2];
    // let nums = vec![1];
    // let nums = vec![1,1,2];
    let nums = vec![1,3];
    // let nums = vec![3,3,3,3];

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

                let mut current_index = (mid % nums.len()) as i32;

                while current_index - 1 >= 0 {

                    let previous_index = (current_index - 1) as usize % nums.len();
                    if nums[current_index as usize] == nums[previous_index] {
                        if result.len() == 1 {
                            result.pop();
                        }

                        result.push(previous_index as i32);
                    }
                    else { break; }

                    current_index -= 1;
                }

                if result.len() == 0 {
                    result.push(mid as i32);
                }

                current_index = (mid % nums.len()) as i32;

                while current_index + 1 < nums.len() as i32 {

                    let next_index = (current_index + 1) as usize % nums.len();

                    if nums[current_index as usize] == nums[next_index] {
                        if result.len() == 2 {
                            result.pop();
                        }

                        result.push(next_index as i32);
                    }
                    else { break; }

                    current_index += 1
                }

                if result.len() == 1 {
                    result.push(mid as i32);
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