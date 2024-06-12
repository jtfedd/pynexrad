use crate::flood_fill::flood_fill::flood_fill;
use crate::flood_fill::flood_filler::FloodFiller;
use crate::model::sweep_data::SweepData;

pub(crate) fn find_regions(vel: &mut SweepData, limits: Vec<f32>) -> (Vec<Vec<i32>>, i32) {
    let mut label = vec![vec![0; vel.gates]; vel.radials];
    let mut feature_count = 0;

    for radial in 0..vel.radials {
        for gate in 0..vel.gates {
            if vel.get_mask(radial, gate) || label[radial][gate] != 0 {
                continue;
            }

            feature_count += 1;
            flood_fill(
                radial,
                gate,
                &mut RegionFiller::new(
                    &mut label,
                    vel,
                    feature_count,
                    &limits,
                    vel.get_value(radial, gate),
                ),
            );
        }
    }

    return (label, feature_count);
}

pub(crate) struct RegionFiller<'a> {
    labels: &'a mut Vec<Vec<i32>>,
    vel: &'a mut SweepData,
    label: i32,
    l_min: f32,
    l_max: f32,
}

impl<'a> RegionFiller<'a> {
    fn new(
        labels: &'a mut Vec<Vec<i32>>,
        vel: &'a mut SweepData,
        label: i32,
        limits: &'a Vec<f32>,
        initial_value: f32,
    ) -> Self {
        let mut l_min = limits[0];
        let mut l_max = limits[1];

        for i in 1..limits.len() {
            l_min = limits[i - 1];
            l_max = limits[i];

            if l_min <= initial_value && initial_value < l_max {
                break;
            }
        }

        RegionFiller {
            labels,
            vel,
            label,
            l_min,
            l_max,
        }
    }
}

impl<'a> FloodFiller for RegionFiller<'a> {
    fn should_fill(&self, radial: usize, gate: usize) -> bool {
        if self.labels[radial][gate] != 0 || self.vel.get_mask(radial, gate) {
            return false;
        }

        let value = self.vel.get_value(radial, gate);

        return self.l_min < value && value < self.l_max;
    }

    fn fill(&mut self, radial: usize, gate: usize) {
        self.labels[radial][gate] = self.label;
    }

    fn radial_max(&self) -> usize {
        self.vel.radials
    }

    fn gate_max(&self) -> usize {
        self.vel.gates
    }
}
