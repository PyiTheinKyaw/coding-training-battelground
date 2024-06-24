use std::collections::HashMap;

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

trait CustomMinHeap {
    fn get_left_child(index: usize) -> usize { (2 * index) + 1 }
    fn get_right_child(index: usize) -> usize { (2 * index) + 2 }
    fn get_parent(index: usize) -> usize { (index - 1) + 2 }

    fn heapify_up(&mut self, index: usize);
    fn sync(&mut self);
    fn swap(&mut self, index_a: usize, index_b: usize);

    fn has_left(&self, index: usize) -> bool;
    fn has_right(&self, index: usize) -> bool;
    
}

impl CustomMinHeap for Snapshot {
    fn sync(&mut self) {
        if self.cache_size != self.local_snap_bar {
            self.local_snap_bar += 1;
            self.heapify_up(self.local_snap_bar - 1);
        }
    }

    fn heapify_up(&mut self, index: usize) {
        if index != 0 {
            let parent = Self::get_parent(index);
            if self.cache[parent].0 < self.cache[index].0 {
                self.swap(parent, index);
                
                self.heapify_up(parent);
            }
        }
    }

    fn swap(&mut self, index_a: usize, index_b: usize) {
        let temp = self.cache[index_a];
        self.cache[index_a] = self.cache[index_b];
        self.cache[index_b] = temp;
    }
    
    fn has_left(&self, index: usize) -> bool {
        Self::get_left_child(index) < self.local_snap_bar
    }
    
    fn has_right(&self, index: usize) -> bool {
        Self::get_right_child(index) < self.local_snap_bar
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
        let _state = &mut self.state;

        if let Some(snapshot) = _state.get_mut(&index) {
            let _index = snapshot.local_snap_bar % snapshot.cache_size;
            snapshot.cache[_index] = (self.snap_indicator, val);
        } else {
            let _snap_id = self.snap_indicator;
            let _snap_size = self.size - self.snap_indicator;

            let mut _cache = vec![(0,0); _snap_size];
            _cache[0] = (_snap_id, val);

            self.state.entry(index).or_insert(
                Snapshot {
                    cache: _cache,
                    local_snap_bar: 0,
                    cache_size: self.size - self.snap_indicator,
                }
            );
        }

        if let Some(count) = self.history.get(&index) {
            self.history.insert(index, *count + 1);
        } else {
            self.history.insert(index, 1);
        } 
    }
    
    fn snap(&mut self) {
        for (k, _) in self.history.iter() {
            if let Some(snap) = self.state.get_mut(k) {
                snap.sync();
            }
        }

        self.snap_indicator += 1;
        self.history.clear();
    }
    
    fn get(&self, index: i32, snap_id: i32) -> i32 {
        todo!()
    }
}



fn main() {
    let mut snapshot_arr = SnapshotArray::new(3); // set the length to be 3
    // snapshot_arr.set(0, 5);
    // snapshot_arr.set(0, 5);
    // snapshot_arr.set(0, 5);
    snapshot_arr.set(1, 5);
    snapshot_arr.snap();
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

    println!("{:?}", snapshot_arr);
    // let snap_id = snapshotArr.snap();  // Take a snapshot, return snap_id = 0


    // println!("snap_id: {:?}", snap_id);

    // snapshotArr.set(0,6);
    // let result = snapshotArr.get(0,0);  // Get the value of array[0] with snap_id = 0, return 5


    // println!("Result: {:?}", result);
}
