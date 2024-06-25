struct SnapshotArray {
    versions: Vec<Vec<(i32, i32)>>,
    snap_id: i32,
}

impl SnapshotArray {
    fn new(length: i32) -> Self {
        let mut versions = vec![vec![]; length as usize];
        SnapshotArray {
            versions,
            snap_id: 0,
        }
    }

    fn set(&mut self, index: i32, val: i32) {
        self.versions[index as usize].push((self.snap_id, val));
    }

    fn snap(&mut self) -> i32 {
        self.snap_id += 1;
        self.snap_id - 1
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let values = &self.versions[index as usize];
        let index = binary_search(values, snap_id + 1);

        if index > 0 {
            values[index - 1].1
        } else {
            0
        }
    }
}

fn binary_search(values: &Vec<(i32, i32)>, first: i32) -> usize {
    let (mut lo, mut hi) = (0, values.len());
    while lo < hi {
        let mid = (lo + hi) / 2;
        if first > values[mid].0 {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }

    lo
}