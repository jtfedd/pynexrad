pub struct SweepData {
    pub radials: usize,
    pub gates: usize,
    data: Vec<Vec<f32>>,
    mask: Vec<Vec<bool>>,
}

impl SweepData {
    pub(crate) fn new(radials: usize, gates: usize) -> Self {
        Self {
            radials,
            gates,
            data: vec![vec![0.0 as f32; gates]; radials],
            mask: vec![vec![true; gates]; radials],
        }
    }

    pub(crate) fn set_value(&mut self, value: f32, radial: usize, gate: usize) {
        self.data[radial][gate] = value;
        self.mask[radial][gate] = false;
    }

    pub(crate) fn set_mask(&mut self, radial: usize, gate: usize) {
        self.mask[radial][gate] = true;
    }

    pub(crate) fn get_value(&self, radial: usize, gate: usize) -> f32 {
        if self.mask[radial][gate] {
            panic!("Value at {} {} is masked", radial, gate)
        }

        return self.data[radial][gate];
    }

    pub(crate) fn get_value_with_fallback(&self, radial: usize, gate: usize, fallback: f32) -> f32 {
        if self.mask[radial][gate] {
            return fallback;
        }

        return self.data[radial][gate];
    }

    pub(crate) fn get_mask(&self, radial: usize, gate: usize) -> bool {
        return self.mask[radial][gate];
    }

    pub(crate) fn min(&self) -> (f32, bool) {
        let mut min = f32::INFINITY;
        let mut has_value = false;

        for r in 0..self.radials {
            for g in 0..self.gates {
                if self.mask[r][g] {
                    continue;
                }

                has_value = true;
                min = f32::min(min, self.data[r][g]);
            }
        }

        return (min, has_value);
    }

    pub(crate) fn max(&self) -> (f32, bool) {
        let mut max = f32::NEG_INFINITY;
        let mut has_value = false;

        for r in 0..self.radials {
            for g in 0..self.gates {
                if self.mask[r][g] {
                    continue;
                }

                has_value = true;
                max = f32::max(max, self.data[r][g]);
            }
        }

        return (max, has_value);
    }
}
