struct SnapshotArray {
    snap: Vec<(usize, i32)>,
    state: Vec<i32>,
}

struct CustomMinHeap {
    
}

impl<'a> SnapshotArray<'a> {
    fn new(length: i32) -> Self {
        SnapshotArray {
            snap: Vec::new(),
            state: vec![0; length as usize]
        }
    }
    
    fn set(&mut self, index: i32, val: i32) {
        let _index = index as usize;
        self.state[_index] = val;
    }
    
    fn snap(&mut self) -> i32 {
        let indexed_snap_list: Vec<(usize, i32)> = self.state
            .iter()
            .enumerate()
            .map(|(i, x)| (i, x as usize) )
            .collect();
    }
    
    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let indexed_snap_list: Vec<(usize, i32)> = self.snap
            .iter()
            .enumerate()
            .map(|(i, x)| (i, x[index as usize]) )
            .collect();

        let mut left = 0;
        let mut right = indexed_snap_list.len();

        while left < right {
            let mid = left + (right - left) / 2;

            if indexed_snap_list[mid].0 == snap_id as usize {
                return indexed_snap_list[mid].1;
            }
            else if indexed_snap_list[mid].0 > snap_id as usize { 
                right = mid;
            }
            else {
                left = mid + 1;
            }            
        }

        0
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
