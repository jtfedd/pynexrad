use crate::model::{sweep_data::SweepData, volume::Volume};
use std::collections::VecDeque;

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

trait FloodFiller {
    fn should_fill(&self, radial: usize, gate: usize) -> bool;
    fn fill(&mut self, radial: usize, gate: usize);
    fn radial_max(&self) -> usize;
    fn gate_max(&self) -> usize;
}

fn flood_fill(
    radial: usize,
    gate: usize,
    filler: &mut impl FloodFiller,
) -> i32 {
    let mut count = 0;

    let mut deq: VecDeque<(usize, usize)> = VecDeque::new();
    deq.push_back((radial, gate));

    while !deq.is_empty() {
        let (r, g) = deq.pop_front().unwrap();

        if !filler.should_fill(r, g) {
            continue;
        }

        filler.fill(r, g);
        count += 1;

        let mut prev_radial = filler.radial_max() - 1;
        if r > 0 {
            prev_radial = r - 1;
        }

        let mut next_radial = 0;
        if r < filler.radial_max() - 1 {
            next_radial = r + 1;
        }

        deq.push_back((prev_radial, g));
        deq.push_back((next_radial, g));
        if g > 0 {
            deq.push_back((r, g-1));
        }
        if g < filler.gate_max() - 1 {
            deq.push_back((r, g+1));
        }
    }

    return count;
}
