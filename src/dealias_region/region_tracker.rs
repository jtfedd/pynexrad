pub(crate) struct RegionTracker {
    node_size: Vec<i32>,
    regions_in_node: Vec<Vec<i32>>,
    pub(crate) unwrap_number: Vec<i32>,
}

impl RegionTracker {
    pub(crate) fn new(region_sizes: &Vec<i32>) -> Self {
        let n_regions = region_sizes.len() + 1;

        let mut node_size = vec![0; n_regions];
        for i in 0..region_sizes.len() {
            node_size[i + 1] = region_sizes[i];
        }

        let mut regions_in_node = vec![vec![0; 0]; n_regions];
        for i in 0..n_regions {
            regions_in_node[i].push(i as i32);
        }

        let unwrap_number = vec![0; n_regions];

        Self {
            node_size,
            regions_in_node,
            unwrap_number,
        }
    }

    pub(crate) fn merge_nodes(&mut self, node_a: i32, node_b: i32) {
        // Merge node b into node a

        // Move all regions from node_b to node_a
        let regions_to_merge = self.regions_in_node[node_b as usize].clone();
        self.regions_in_node[node_a as usize].extend(regions_to_merge);
        self.regions_in_node[node_b as usize].clear();

        // Update node sizes
        self.node_size[node_a as usize] += self.node_size[node_b as usize];
        self.node_size[node_b as usize] = 0;
    }

    pub(crate) fn unwrap_node(&mut self, node: i32, nwrap: i32) {
        // Unwrap all gates contained in a node
        if nwrap == 0 {
            return;
        }

        // For each region in the node add nwrap
        for i in 0..self.regions_in_node[node as usize].len() {
            self.unwrap_number[self.regions_in_node[node as usize][i] as usize] += nwrap;
        }
    }

    pub(crate) fn apply_offset(&mut self, offset: i32) {
        for i in 0..self.unwrap_number.len() {
            self.unwrap_number[i] -= offset;
        }
    }

    pub(crate) fn get_node_size(&self, node: i32) -> i32 {
        return self.node_size[node as usize];
    }
}
