pub(crate) struct EdgeTracker {
    node_alpha: Vec<i32>,
    node_beta: Vec<i32>,
    sum_diff: Vec<f32>,
    weight: Vec<i32>,
    common_finder: Vec<bool>,
    common_index: Vec<i32>,
    last_base_node: i32,
    edges_in_node: Vec<Vec<i32>>,
}

impl EdgeTracker {
    pub(crate) fn new(
        indices: (Vec<i32>, Vec<i32>),
        edge_count: Vec<i32>,
        velocities: (Vec<f32>, Vec<f32>),
        nyquist_interval: f32,
        nnodes: i32,
    ) -> Self {
        let (idx1, idx2) = indices;
        let (vel1, vel2) = velocities;

        let nedges = idx1.len() / 2;

        let mut node_alpha = vec![0; nedges];
        let mut node_beta = vec![0; nedges];
        let mut sum_diff = vec![0.0; nedges];
        let mut weight = vec![0; nedges];
        let common_finder = vec![false; nnodes as usize];
        let common_index = vec![0; nnodes as usize];
        let last_base_node = -1;
        let mut edges_in_node = vec![vec![0; 0]; nnodes as usize];

        let mut edge: usize = 0;

        for index in 0..idx1.len() {
            let i = idx1[index];
            let j = idx2[index];
            let count = edge_count[index];
            let vel = vel1[index];
            let nvel = vel2[index];

            if i < j {
                continue;
            }

            node_alpha[edge] = i;
            node_beta[edge] = j;
            sum_diff[edge] = (vel - nvel) / nyquist_interval;
            weight[edge] = count;
            edges_in_node[i as usize].push(edge as i32);
            edges_in_node[j as usize].push(edge as i32);

            edge += 1;
        }

        Self {
            node_alpha,
            node_beta,
            sum_diff,
            weight,
            common_finder,
            common_index,
            last_base_node,
            edges_in_node,
        }
    }

    pub(crate) fn merge_nodes(&mut self, base_node: i32, merge_node: i32, foo_edge: i32) {
        // Remove edge between base and merge nodes
        self.weight[foo_edge as usize] = -999;

        let idx_to_remove_merge_node = self.edges_in_node[merge_node as usize]
            .iter()
            .position(|a| *a == foo_edge)
            .unwrap();
        self.edges_in_node[merge_node as usize].swap_remove(idx_to_remove_merge_node);

        let idx_to_remove_base_node = self.edges_in_node[base_node as usize]
            .iter()
            .position(|a| *a == foo_edge)
            .unwrap();
        self.edges_in_node[base_node as usize].swap_remove(idx_to_remove_base_node);

        self.common_finder[merge_node as usize] = false;

        // Find all the edges in the two nodes
        let edges_in_merge = self.edges_in_node[merge_node as usize].clone();

        // Loop over base_node edges if last base_node was different
        if self.last_base_node != base_node {
            for i in 0..self.common_finder.len() {
                self.common_finder[i] = false;
            }

            let edges_in_base = self.edges_in_node[base_node as usize].clone();
            for edge_num in edges_in_base {
                // Reverse edge if needed so node_alpha is base_node
                if self.node_beta[edge_num as usize] == base_node {
                    self.reverse_edge_direction(edge_num as usize);
                }
                assert_eq!(self.node_alpha[edge_num as usize], base_node);

                // Find all neighboring nodes to base_node
                let neighbor = self.node_beta[edge_num as usize];
                self.common_finder[neighbor as usize] = true;
                self.common_index[neighbor as usize] = edge_num;
            }
        }

        // Loop over edge nodes
        for edge_num in edges_in_merge {
            // Reverse edge if needed so that node alpha is merge_node
            if self.node_beta[edge_num as usize] == merge_node {
                self.reverse_edge_direction(edge_num as usize);
            }
            assert_eq!(self.node_alpha[edge_num as usize], merge_node);

            // Update all the edges to point to the base node
            self.node_alpha[edge_num as usize] = base_node;

            let neighbor = self.node_beta[edge_num as usize];
            if self.common_finder[neighbor as usize] {
                // If base_node also has an edge with the neighbor combine them
                let base_edge_num = self.common_index[neighbor as usize].clone();
                self.combine_edges(base_edge_num, edge_num, merge_node, neighbor);
            } else {
                // If not fill in common arrays
                self.common_finder[neighbor as usize] = true;
                self.common_index[neighbor as usize] = edge_num;
            }
        }

        // Move all edges from merge_node to base_node
        let edges = self.edges_in_node[merge_node as usize].clone();
        self.edges_in_node[base_node as usize].extend(edges);
        self.edges_in_node[merge_node as usize].clear();
        self.last_base_node = base_node;
    }

    fn combine_edges(
        &mut self,
        base_edge: i32,
        merge_edge: i32,
        merge_node: i32,
        neighbor_node: i32,
    ) {
        // Combine edge weights
        self.weight[base_edge as usize] += self.weight[merge_edge as usize];
        self.weight[merge_edge as usize] = -999;

        // Combine sums
        self.sum_diff[base_edge as usize] += self.sum_diff[merge_edge as usize];

        // Remove merge_edge from both node lists
        let idx_to_remove_merge_node = self.edges_in_node[merge_node as usize]
            .iter()
            .position(|a| *a == merge_edge)
            .unwrap();
        self.edges_in_node[merge_node as usize].swap_remove(idx_to_remove_merge_node);

        let idx_to_remove_neighbor_node = self.edges_in_node[neighbor_node as usize]
            .iter()
            .position(|a| *a == merge_edge)
            .unwrap();
        self.edges_in_node[neighbor_node as usize].swap_remove(idx_to_remove_neighbor_node);
    }

    fn reverse_edge_direction(&mut self, edge: usize) {
        // swap nodes
        let old_alpha = self.node_alpha[edge];
        let old_beta = self.node_beta[edge];
        self.node_alpha[edge] = old_beta;
        self.node_beta[edge] = old_alpha;
        // swap sums
        self.sum_diff[edge] = -1.0 * self.sum_diff[edge]
    }

    pub(crate) fn unwrap_node(&mut self, node: i32, nwrap: i32) {
        if nwrap == 0 {
            return;
        }

        for i in 0..self.edges_in_node[node as usize].len() {
            let edge = self.edges_in_node[node as usize][i] as usize;
            let weight = self.weight[edge];
            if node == self.node_alpha[edge] {
                self.sum_diff[edge] += (weight * nwrap) as f32;
            } else {
                assert_eq!(self.node_beta[edge], node);
                self.sum_diff[edge] += ((-weight) * nwrap) as f32;
            }
        }
    }

    pub(crate) fn pop_edge(&self) -> (bool, (i32, i32, i32, f32, i32)) {
        let mut edge_num = 0;
        let mut max_weight = self.weight[0];

        for i in 0..self.weight.len() {
            if self.weight[i] > max_weight {
                edge_num = i as i32;
                max_weight = self.weight[i];
            }
        }

        let node1 = self.node_alpha[edge_num as usize];
        let node2 = self.node_beta[edge_num as usize];
        let weight = self.weight[edge_num as usize];
        let diff = self.sum_diff[edge_num as usize] / (weight as f32);

        return (weight < 0, (node1, node2, weight, diff, edge_num));
    }
}
