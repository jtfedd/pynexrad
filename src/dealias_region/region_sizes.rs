pub(crate) fn region_sizes(labels: &Vec<Vec<i32>>, feature_count: i32) -> (i32, Vec<i32>) {
    let mut num_masked_gates = 0;
    let mut region_sizes = vec![0; feature_count as usize];

    for i in 0..labels.len() {
        for j in 0..labels[i].len() {
            if labels[i][j] == 0 {
                num_masked_gates += 1;
            } else {
                region_sizes[(labels[i][j] as usize) - 1] += 1;
            }
        }
    }

    return (num_masked_gates, region_sizes);
}
