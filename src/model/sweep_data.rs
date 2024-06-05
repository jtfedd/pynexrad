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
            mask: vec![vec![false; gates]; radials],
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

    pub(crate) fn get_mask(&self, radial: usize, gate: usize) -> bool {
        return self.mask[radial][gate];
    }
}
