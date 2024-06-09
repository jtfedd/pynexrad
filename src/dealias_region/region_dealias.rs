use crate::dealias_region::find_edges::find_edges;
use crate::dealias_region::find_regions::find_regions;
use crate::dealias_region::interval_limits::find_interval_limits;
use crate::dealias_region::region_sizes::region_sizes;
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

        let interval_limits = find_interval_limits(vel, sweep.nyquist_vel, interval_splits);
        let (labels, feature_count) = find_regions(vel, interval_limits);
        if feature_count < 2 {
            continue;
        }

        let (num_masked_gates, region_sizes) = region_sizes(&labels, feature_count);
        let (indices, edge_count, velos) = find_edges(
            labels,
            num_masked_gates,
            vel,
            skip_between_rays,
            skip_along_ray,
        );
    }
}
