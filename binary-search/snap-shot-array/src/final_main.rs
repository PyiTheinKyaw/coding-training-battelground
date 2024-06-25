use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(Debug)]
struct SnapshotArray {
    size: usize,
    snap_indicator: usize,
    state: HashMap<usize, Snapshot>,
    history: HashMap<usize, usize>,
}

#[derive(Debug)]
struct Snapshot {
    local_snap_bar: usize,
    cache_size: usize,
    cache: Vec<(usize, i32)>,
}

impl Snapshot {
    fn sync(&mut self) {
        if self.cache_size != self.local_snap_bar {
            self.local_snap_bar += 1;
        }
    }

    fn binary_search(&self, snap_id: usize) -> i32 {
    
        let mut left = 0;
        let mut right = self.cache.len();

        while left < right {
            let mid = left + (right - left) / 2;

            match self.cache[mid].0.cmp(&snap_id) {
                Ordering::Equal => { return self.cache[mid].1 },
                Ordering::Greater => right = mid,
                Ordering::Less => left = mid + 1,
            }

        }
       
       -1
    }
}

impl SnapshotArray {

    fn new(size: usize) -> Self {
        SnapshotArray {
            snap_indicator: 0,
            size,
            state: HashMap::with_capacity(size),
            history: HashMap::new()
        }
    }

    fn set(&mut self, index: i32, val: i32) {
        let index = index as usize;

        if let Some(snapshot) = self.state.get_mut(&index) {

            // update following logic to build binary tree.
            let _index: usize = snapshot.local_snap_bar % snapshot.cache_size;
            if _index < snapshot.cache.len() {
                snapshot.cache[_index] = (self.snap_indicator, val);
            } else {
                snapshot.cache.push((self.snap_indicator, val));
            }
        } else {
            let snapshot = Snapshot {
                cache: vec![(self.snap_indicator, val)],
                local_snap_bar: 0,
                cache_size: self.size - self.snap_indicator,
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

        self.snap_indicator as i32
    }
    
    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let index = index as usize;
        let snap_id = snap_id as usize;
        
        if let Some(snap) = self.state.get(&index) {
            return snap.binary_search(snap_id);
        }

        -1
    }
}

fn main() {
    let mut snapshot_arr = SnapshotArray::new(6); // set the length to be 3

    snapshot_arr.set(1, 1);

    snapshot_arr.snap();
    snapshot_arr.set(1, 3);

    snapshot_arr.snap();
    snapshot_arr.snap();
    snapshot_arr.snap();
    snapshot_arr.snap();
    snapshot_arr.set(1, 6);

    snapshot_arr.snap();
    snapshot_arr.snap();
    snapshot_arr.snap();
    snapshot_arr.snap();
    snapshot_arr.snap();
    snapshot_arr.set(1, 600);

    snapshot_arr.snap();
    snapshot_arr.set(1, 900);

    snapshot_arr.snap();

    let result1 = snapshot_arr.get(1, 11);

    println!("{:?}", snapshot_arr);
    println!("result1 {:?}", result1);
}
