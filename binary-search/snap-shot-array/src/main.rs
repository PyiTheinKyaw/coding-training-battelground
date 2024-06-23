use std::collections::HashMap;

struct SnapshotArray {
    snapshot_cache: HashMap<usize, Vec<(usize, i32)>>,
    current_snapshot: usize,
    cache_size: usize
}

trait CustomMinHeap {
    fn get_left_child(index: usize) -> usize { (2 * index) + 1 }
    fn get_right_child(index: usize) -> usize { (2 * index) + 2 }
    fn get_parent(index: usize) -> usize { (index - 1) + 2 }

    fn heapify_up(index: usize);
    fn swap(&mut self, index_a: usize, index_b: usize);
    fn has_left(&self, index: usize) -> bool;
    fn has_right(&self, index: usize) -> bool;
}

impl CustomMinHeap for SnapshotArray{
    fn heapify_up(index: usize) {
        todo!()
    }

    fn swap(&mut self, index_a: usize, index_b: usize) {
        let temp = self.snapshot_cache[index_a];
        self.snapshot_cache[index_a] = self.snapshot_cache[index_b];
        self.snapshot_cache[index_b] = temp;
    }
    
    fn has_left(&self, index: usize) -> bool {
        Self::get_left_child(index) < self.current_snapshot
    }
    
    fn has_right(&self, index: usize) -> bool {
        Self::get_right_child(index) < self.current_snapshot
    }
}
    

impl SnapshotArray {

    fn new(size: usize) -> Self {
        SnapshotArray {
            snapshot_cache: HashMap::with_capacity(size),
            current_snapshot: 0,
            cache_size: size,
        }
    }

    fn build_snap(&mut self, index: usize, value: i32) {
        if !(self.current_snapshot == self.cache_size) {
            let _snapshot_id = self.current_snapshot;
            let _snapshot_cache = &mut self.snapshot_cache;

            _snapshot_cache
                .entry(index)
                .or_insert_with(|| Vec::new()[self.current_snapshot] = (_snapshot_id, value));

            self.current_snapshot += 1;
        }
    }

    fn set(&mut self, index: i32, val: i32) {

        let index = index as usize;

        if let Some(snap_array) = self.snapshot_cache.get(&index) {
            

            snap_array[index] = (val;

        } else { self.build_snap(index , val); }
    }
    
    fn snap(&mut self) -> i32 {
        todo!()
    }
    
    fn get(&self, index: i32, snap_id: i32) -> i32 {
        todo!()
    }
}



fn main() {
    let mut snapshotArr = SnapshotArray::new(3); // set the length to be 3
    snapshotArr.set(0,5);  // Set array[0] = 5
    let snap_id = snapshotArr.snap();  // Take a snapshot, return snap_id = 0


    println!("snap_id: {:?}", snap_id);

    snapshotArr.set(0,6);
    let result = snapshotArr.get(0,0);  // Get the value of array[0] with snap_id = 0, return 5


    println!("Result: {:?}", result);
}
