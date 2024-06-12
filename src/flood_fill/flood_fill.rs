use std::collections::VecDeque;

use crate::flood_fill::flood_filler::FloodFiller;

pub(crate) fn flood_fill(radial: usize, gate: usize, filler: &mut impl FloodFiller) -> i32 {
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
            deq.push_back((r, g - 1));
        }
        if g < filler.gate_max() - 1 {
            deq.push_back((r, g + 1));
        }
    }

    return count;
}
