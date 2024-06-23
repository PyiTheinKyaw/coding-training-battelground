use std::collections::HashMap;

struct SnapshotArray {
    snap: Vec<HashMap<i32, i32>>,
    state: HashMap<i32, i32>
}

impl SnapshotArray {
    fn new(length: i32) -> Self {
        SnapshotArray {
            snap: Vec::with_capacity(length as usize),
            state: HashMap::with_capacity(length as usize)
        }
    }
    
    fn set(&mut self, index: i32, val: i32) {
        self.state.insert(index, val);
    }
    
    fn snap(&mut self) -> i32 {
        self.snap.push(self.state.clone());
        (self.snap.len() - 1) as i32
    }
    
    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let snap_index = snap_id as usize % self.snap.len();
        if let Some(snap_point) = self.snap.get(snap_index) {
            
            if let Some(snap_state) = snap_point.get(&index) {
                return *snap_state;
            }

            return 0;
        }

        0
    }
}