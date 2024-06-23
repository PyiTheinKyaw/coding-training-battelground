use std::collections::HashMap;
use std::cmp::Ordering;

struct TimeMap {
    store: HashMap<String, Vec<(i32, String)>>,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {

    fn new() -> Self {
        TimeMap { store: HashMap::new() }
    }
    
    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.store.entry(key).or_default().push((timestamp, value));
    }
    
    fn get(&self, key: String, timestamp: i32) -> String {

        if let Some(values) = self.store.get(&key) {
            let mut left = 0;
            let mut right = values.len();

            while left < right {

                let mid = left + (right - left) / 2;

                match timestamp.cmp(&values[mid].0) {
                    Ordering::Equal => { return values[mid].1.clone() },
                    Ordering::Greater => left = mid + 1 ,
                    Ordering::Less => right = mid
                }
            }

            if right == 0 {
                return "".to_string();
            }

            let last_timestamp = (right - 1) % values.len();
            return values[last_timestamp].1.clone();
        }

        "".to_string()
    }
}





fn main() {
    let mut time_map = TimeMap::new();
    time_map.set("foo".to_string(), "bar".to_string(), 1);
    println!("1) {}", time_map.get("foo".to_string(), 1)); // Output: "bar"
    println!("3) {}", time_map.get("foo".to_string(), 3)); // Output: "bar"
    time_map.set("foo".to_string(), "bar2".to_string(), 4);
    println!("4) {}", time_map.get("foo".to_string(), 4)); // Output: "bar2"
    println!("5) {}", time_map.get("foo".to_string(), 5)); // Output: "bar2"
}