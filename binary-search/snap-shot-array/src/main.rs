use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(Debug)]
struct SnapshotArray {
    snap_indicator: usize,
    state: HashMap<usize, Snapshot>,
    history: HashMap<usize, usize>,
}

#[derive(Debug)]
struct Snapshot {
    local_snap_bar: usize,
    cache: Vec<(usize, i32)>,
}

impl Snapshot {
    fn sync(&mut self) {
        self.local_snap_bar += 1;
    }

    fn binary_search(&self, snap_id: usize) -> i32 {
    
        let mut left = 0;
        let mut right = self.cache.len();

        while left < right {
            let mid = left + (right - left) / 2;

            match self.cache[mid].0.cmp(&snap_id) {
                Ordering::Equal => { return self.cache[mid].1 },
                Ordering::Greater => right = mid ,
                Ordering::Less => left = mid + 1,
            }
        }      

        if left != 0 && left == right {
            return self.cache[left - 1].1;
        }

       0
    }
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SnapshotArray {

    fn new(length: i32) -> Self {
        let length = length as usize;

        SnapshotArray {
            snap_indicator: 0,
            state: HashMap::with_capacity(length),
            history: HashMap::new()
        }
    }
    
    fn set(&mut self, index: i32, val: i32) {
        let index = index as usize;

        if let Some(snapshot) = self.state.get_mut(&index) {

            let _index: usize = snapshot.local_snap_bar;
            if _index < snapshot.cache.len() {
                snapshot.cache[_index] = (self.snap_indicator, val);
            } else {
                snapshot.cache.push((self.snap_indicator, val));
            }

        } else {
            let snapshot = Snapshot {
                cache: vec![(self.snap_indicator, val)],
                local_snap_bar: 0
            };

            self.state.insert(index, snapshot);
        }

        if let Some(count) = self.history.get(&index) {
            self.history.insert(index, *count + 1);
        } else {
            self.history.insert(index, 1);
        } 
    }
    
    fn snap(&mut self) -> i32 {
        for cahced_index in self.history.iter() {
            if let Some(snap) = self.state.get_mut(cahced_index.0) {
                snap.sync();
            }
        }

        self.snap_indicator += 1;
        self.history.clear();

        (self.snap_indicator - 1) as i32
    }
    
    
    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let index = index as usize;
        let snap_id = snap_id as usize;
        
        if let Some(snap) = self.state.get(&index) {
            return snap.binary_search(snap_id);
        }

        0
    }
}


fn main() {
    let mut snapshot_arr = SnapshotArray::new(2); // set the length to be 3

    snapshot_arr.snap();
    snapshot_arr.set(0, 15);

    snapshot_arr.get(1, 0);
    snapshot_arr.get(0, 0);

    println!("{:?}", snapshot_arr);
    println!("get {:?}", snapshot_arr.get(0, 3));
}