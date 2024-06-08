use crate::{flood_fill::{flood_fill::flood_fill, flood_filler::FloodFiller}, model::{sweep_data::SweepData, volume::Volume}};

// Removes all isolated groups of velocity data which have
// a total number of gates less than the threshold
pub(crate) fn despeckle(volume: &mut Volume, threshold: i32) {
    for sweep in volume.sweeps.iter_mut() {
        if sweep.velocity.is_none() {
            continue;
        }

        let vel = sweep.velocity.as_mut().unwrap();
        let mut processed = vec![vec![0 as u8; vel.gates]; vel.radials];

        for radial in 0..vel.radials {
            for gate in 0..vel.gates {
                // Don't process values we have already processed
                if processed[radial][gate] != 0 {
                    continue;
                }

                // Don't process masked values, but mark them as processed
                if vel.get_mask(radial, gate) {
                    processed[radial][gate] = 1 as u8;
                    continue;
                }

                let count = flood_fill(
                    radial,
                    gate,
                    &mut SearchingFiller::new(&mut processed, vel),
                );

                flood_fill(
                    radial,
                    gate,
                    &mut ResultFiller::new(&mut processed, vel, count < threshold),
                );
            }
        }
    }
}

struct ResultFiller<'a> {
    processed: &'a mut Vec<Vec<u8>>,
    vel: &'a mut SweepData,
    mask: bool,
}

impl<'a> ResultFiller<'a> {
    fn new(processed: &'a mut Vec<Vec<u8>>, vel: &'a mut SweepData, mask: bool) -> Self {
        ResultFiller { processed, vel, mask }
    }
}

impl<'a> FloodFiller for ResultFiller<'a> {
    fn should_fill(&self, radial: usize, gate: usize) -> bool {
        self.processed[radial][gate] == 2
    }

    fn fill(&mut self, radial: usize, gate: usize) {
        self.processed[radial][gate] = 1 as u8;
        if self.mask {
            self.vel.set_mask(radial, gate);
        }
    }

    fn radial_max(&self) -> usize {
        self.vel.radials
    }

    fn gate_max(&self) -> usize {
        self.vel.gates
    }
}

struct SearchingFiller<'a> {
    processed: &'a mut Vec<Vec<u8>>,
    vel: &'a mut SweepData,
}

impl<'a> SearchingFiller<'a> {
    fn new(processed: &'a mut Vec<Vec<u8>>, vel: &'a mut SweepData) -> Self {
        SearchingFiller { processed, vel }
    }
}

impl<'a> FloodFiller for SearchingFiller<'a> {
    fn should_fill(&self, radial: usize, gate: usize) -> bool {
        self.processed[radial][gate] == 0 && !self.vel.get_mask(radial, gate)
    }

    fn fill(&mut self, radial: usize, gate: usize) {
        self.processed[radial][gate] = 2 as u8;
    }

    fn radial_max(&self) -> usize {
        self.vel.radials
    }

    fn gate_max(&self) -> usize {
        self.vel.gates
    }
}
