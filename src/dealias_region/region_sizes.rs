pub(crate) fn region_sizes(labels: &Vec<Vec<i32>>, feature_count: i32) -> Vec<i32> {
    let mut region_sizes = vec![0; feature_count as usize];

    for i in 0..labels.len() {
        for j in 0..labels[i].len() {
            if labels[i][j] != 0 {
                region_sizes[(labels[i][j] as usize) - 1] += 1;
            }
        }
    }

    return region_sizes;
}
