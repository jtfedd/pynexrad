use crate::model::sweep_data::SweepData;

pub(crate) fn find_edges(
    labels: &Vec<Vec<i32>>,
    data: &SweepData,
    skip_between_rays: i32,
    skip_along_ray: i32,
) -> ((Vec<i32>, Vec<i32>), Vec<i32>, (Vec<f32>, Vec<f32>)) {
    let ((index1, index2), (vel1, vel2)) =
        edge_finder(labels, data, skip_between_rays, skip_along_ray);

    // Return early if edges were not found
    if vel1.is_empty() {
        return (
            ([].to_vec(), [].to_vec()),
            [].to_vec(),
            ([].to_vec(), [].to_vec()),
        );
    }

    // Deduplicate edges and sum values of duplicate edges
    let mut order = vec![0; vel1.len()];
    for i in 0..vel1.len() {
        order[i] = i;
    }

    order.sort_unstable_by(|a, b| (index1[*a], index2[*a]).cmp(&(index1[*b], index2[*b])));

    let mut index1_result: Vec<i32> = Vec::new();
    let mut index2_result: Vec<i32> = Vec::new();
    let mut count: Vec<i32> = Vec::new();
    let mut vel1_result: Vec<f32> = Vec::new();
    let mut vel2_result: Vec<f32> = Vec::new();

    index1_result.push(index1[order[0]]);
    index2_result.push(index2[order[0]]);
    count.push(1);
    vel1_result.push(vel1[order[0]]);
    vel2_result.push(vel2[order[0]]);
    let mut unique = 1;

    for i in 1..vel1.len() {
        let o = order[i];
        let i1 = index1[o];
        let i2 = index2[o];

        if index1_result[unique - 1] == i1 && index2_result[unique - 1] == i2 {
            // This is a duplicate of the previous one
            // Add to the previous one

            count[unique - 1] += 1;
            vel1_result[unique - 1] += vel1[o];
            vel2_result[unique - 1] += vel2[o];
        } else {
            unique += 1;

            index1_result.push(i1);
            index2_result.push(i2);
            count.push(1);
            vel1_result.push(vel1[o]);
            vel2_result.push(vel2[o]);
        }
    }

    return (
        (index1_result, index2_result),
        count,
        (vel1_result, vel2_result),
    );
}

fn edge_finder(
    labels: &Vec<Vec<i32>>,
    data: &SweepData,
    skip_between_rays: i32,
    skip_along_ray: i32,
) -> ((Vec<i32>, Vec<i32>), (Vec<f32>, Vec<f32>)) {
    let mut collector = EdgeCollector::new();

    let right = data.radials as i32 - 1;
    let bottom = data.gates as i32 - 1;

    for radial in 0..data.radials {
        for gate in 0..data.gates {
            let label = labels[radial][gate];
            if label == 0 {
                continue;
            }

            let vel = data.get_value(radial, gate);
            let x_index = radial as i32;
            let y_index = gate as i32;

            // left
            {
                let mut x_check = x_index - 1;
                if x_check == -1 {
                    x_check = right;
                }

                let mut neighbor = labels[x_check as usize][y_index as usize];
                let mut nvel =
                    data.get_value_with_fallback(x_check as usize, y_index as usize, 0.0);

                // If the left side gate is masked, keep looking to the left
                // until we find a valid gate or reach the maximum gap size
                if neighbor == 0 {
                    for _ in 0..skip_between_rays {
                        x_check -= 1;
                        if x_check == -1 {
                            x_check = right;
                        }

                        neighbor = labels[x_check as usize][y_index as usize];
                        nvel =
                            data.get_value_with_fallback(x_check as usize, y_index as usize, 0.0);
                        if neighbor != 0 {
                            break;
                        }
                    }
                }

                collector.add_edge(label, neighbor, vel, nvel);
            }

            // right
            {
                let mut x_check = x_index + 1;
                if x_check == right + 1 {
                    x_check = 0;
                }

                let mut neighbor = labels[x_check as usize][y_index as usize];
                let mut nvel =
                    data.get_value_with_fallback(x_check as usize, y_index as usize, 0.0);

                // If the right side gate is masked, keep looking to the right
                // until we find a valid gate or reach the maximum gap size
                if neighbor == 0 {
                    for _ in 0..skip_between_rays {
                        x_check += 1;
                        if x_check == right + 1 {
                            x_check = 0;
                        }

                        neighbor = labels[x_check as usize][y_index as usize];
                        nvel =
                            data.get_value_with_fallback(x_check as usize, y_index as usize, 0.0);
                        if neighbor != 0 {
                            break;
                        }
                    }
                }

                collector.add_edge(label, neighbor, vel, nvel);
            }

            // top
            {
                let mut y_check = y_index - 1;
                if y_check != -1 {
                    let mut neighbor = labels[x_index as usize][y_check as usize];
                    let mut nvel =
                        data.get_value_with_fallback(x_index as usize, y_check as usize, 0.0);

                    // If the top side gate is masked, keep looking up
                    // until we find a valid gate or reach the maximum gap size
                    if neighbor == 0 {
                        for _ in 0..skip_along_ray {
                            y_check -= 1;
                            if y_check == -1 {
                                break;
                            }

                            neighbor = labels[x_index as usize][y_check as usize];
                            nvel = data.get_value_with_fallback(
                                x_index as usize,
                                y_check as usize,
                                0.0,
                            );

                            if neighbor != 0 {
                                break;
                            }
                        }
                    }

                    collector.add_edge(label, neighbor, vel, nvel);
                }
            }

            // bottom
            {
                let mut y_check = y_index + 1;
                if y_check != bottom + 1 {
                    let mut neighbor = labels[x_index as usize][y_check as usize];
                    let mut nvel =
                        data.get_value_with_fallback(x_index as usize, y_check as usize, 0.0);

                    // If the top side gate is masked, keep looking up
                    // until we find a valid gate or reach the maximum gap size
                    if neighbor == 0 {
                        for _ in 0..skip_along_ray {
                            y_check += 1;
                            if y_check == bottom + 1 {
                                break;
                            }

                            neighbor = labels[x_index as usize][y_check as usize];
                            nvel = data.get_value_with_fallback(
                                x_index as usize,
                                y_check as usize,
                                0.0,
                            );

                            if neighbor != 0 {
                                break;
                            }
                        }
                    }

                    collector.add_edge(label, neighbor, vel, nvel);
                }
            }
        }
    }

    return (
        (collector.label, collector.neighbor),
        (collector.l_vel, collector.n_vel),
    );
}

struct EdgeCollector {
    label: Vec<i32>,
    neighbor: Vec<i32>,
    l_vel: Vec<f32>,
    n_vel: Vec<f32>,
}

impl EdgeCollector {
    fn new() -> Self {
        Self {
            label: Vec::new(),
            neighbor: Vec::new(),
            l_vel: Vec::new(),
            n_vel: Vec::new(),
        }
    }

    fn add_edge(&mut self, label: i32, neighbor: i32, vel: f32, nvel: f32) {
        if label == neighbor || neighbor == 0 {
            return;
        }

        self.label.push(label);
        self.neighbor.push(neighbor);
        self.l_vel.push(vel);
        self.n_vel.push(nvel);
    }
}
