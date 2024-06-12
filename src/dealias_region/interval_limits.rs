use crate::model::sweep_data::SweepData;

pub(crate) fn find_interval_limits(
    vel: &SweepData,
    nyquist: f32,
    interval_splits: i32,
) -> Vec<f32> {
    let interval_size = (2.0 * nyquist) / (interval_splits as f32);

    // If the min or max values do not fall in the nyquist interval then add additional
    // splits to cover all of the values.
    let mut add_start = 0;
    let mut add_end = 0;

    let (max_vel, has_value) = vel.max();
    if has_value && max_vel > nyquist {
        // Velocities outside of nyquist interval
        // println!(
        //     "Velocities outside of nyquist interval: {}, {}",
        //     nyquist, max_vel
        // );
        add_start = f32::ceil((max_vel - nyquist) / interval_size) as i32;
    }

    let (min_vel, has_value) = vel.min();
    if has_value && min_vel < -nyquist {
        // Velocities outside of nyquist interval
        // println!(
        //     "Velocities outside of nyquist interval: {}, {}",
        //     nyquist, min_vel
        // );
        add_end = f32::ceil(-(min_vel + nyquist) / interval_size) as i32;
    }

    // Start of the first interval
    let start = -nyquist - (add_start as f32 * interval_size);
    let interval_count = add_start + add_end + interval_splits;

    let mut result = vec![start];
    for i in 0..interval_count {
        result.push(start + ((i + 1) as f32 * interval_size));
    }

    return result;
}
