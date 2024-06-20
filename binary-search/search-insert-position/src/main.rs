fn main() {
    let nums = vec![1,3,5,6];
    let target = 7;
    let result = search_insert(nums, target);

    println!("{:?}", result);
}


pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    if !(1 <= nums.len() && nums.len() <= 10i32.pow(4) as usize) { return -1 }
    if !(target < 10i32.pow(4)) {return -1 }

    use std::cmp::Ordering;

    let (mut low, mut high, mut optical_index) = (0, nums.len(), 0);
    
    while low < high {
        let mid = ((high-low) / 2) + low;
        
        if !(nums[mid] > -10i32.pow(4)) { return -1 }

        match nums[mid].cmp(&target) {
            Ordering::Equal => return mid as i32,
            Ordering::Less => low = mid + 1,
            Ordering::Greater => high = mid,
        }

        if low == high {
            optical_index = low;
        }
    }

    optical_index as i32
}