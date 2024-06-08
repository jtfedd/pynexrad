use crate::model::volume::Volume;

// Masks all velocity values where the corresponding reflectivity value is
// less than the specified threshold
pub(crate) fn apply_reflectivity_threshold(volume: &mut Volume, threshold: f32) {
    for sweep in volume.sweeps.iter_mut() {
        if sweep.velocity.is_none() {
            continue;
        }

        let vel = sweep.velocity.as_mut().unwrap();
        let refl = sweep.reflectivity.as_ref().unwrap();

        for radial in 0..vel.radials {
            for gate in 0..vel.gates {
                if vel.get_mask(radial, gate) {
                    continue;
                }

                if refl.get_mask(radial, gate) {
                    vel.set_mask(radial, gate);
                    continue;
                }

                if refl.get_value(radial, gate) < threshold {
                    vel.set_mask(radial, gate);
                    continue;
                }
            }
        }
    }
}
