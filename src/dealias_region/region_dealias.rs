use crate::dealias_region::combine_regions::combine_regions;
use crate::dealias_region::edge_tracker::EdgeTracker;
use crate::dealias_region::find_edges::find_edges;
use crate::dealias_region::find_regions::find_regions;
use crate::dealias_region::interval_limits::find_interval_limits;
use crate::dealias_region::region_sizes::region_sizes;
use crate::dealias_region::region_tracker::RegionTracker;
use crate::model::volume::Volume;

pub(crate) fn dealias_region_based(
    volume: &mut Volume,
    interval_splits: i32,   // default 3
    skip_between_rays: i32, // default 100
    skip_along_ray: i32,    // default 100
    centered: bool,         // default true
) {
    for sweep in volume.sweeps.iter_mut() {
        if sweep.velocity.is_none() {
            continue;
        }
        let vel = sweep.velocity.as_mut().unwrap();
        let nyquist_interval = 2.0 * sweep.nyquist_vel;

        let interval_limits = find_interval_limits(vel, sweep.nyquist_vel, interval_splits);
        let (labels, feature_count) = find_regions(vel, interval_limits);
        if feature_count < 2 {
            continue;
        }

        let region_sizes = region_sizes(&labels, feature_count);
        let (indices, edge_count, velos) =
            find_edges(&labels, vel, skip_between_rays, skip_along_ray);

        if edge_count.is_empty() {
            continue;
        }

        let mut region_tracker = RegionTracker::new(&region_sizes);
        let mut edge_tracker = EdgeTracker::new(
            indices,
            edge_count,
            velos,
            nyquist_interval,
            feature_count + 1,
        );

        loop {
            if combine_regions(&mut region_tracker, &mut edge_tracker) {
                break;
            }
        }

        if centered {
            let mut gates_dealiased = 0;
            for i in 0..region_sizes.len() {
                gates_dealiased += region_sizes[i];
            }

            let mut total_folds = 0;
            for i in 0..region_sizes.len() {
                total_folds += region_sizes[i] * region_tracker.unwrap_number[i + 1];
            }

            let sweep_offset = (total_folds as f32 / gates_dealiased as f32).round() as i32;

            if sweep_offset != 0 {
                region_tracker.apply_offset(sweep_offset);
            }
        }

        // Dealias the data using the fold numbers
        for r in 0..vel.radials {
            for g in 0..vel.gates {
                if vel.get_mask(r, g) {
                    continue;
                }

                let label = labels[r][g];
                let nwrap = region_tracker.unwrap_number[label as usize];
                let vel_uncorr = vel.get_value(r, g);
                let corrected = vel_uncorr + (nwrap as f32 * nyquist_interval);
                vel.set_value(corrected, r, g);
            }
        }
    }
}
