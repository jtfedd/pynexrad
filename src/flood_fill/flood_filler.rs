pub(crate) trait FloodFiller {
    fn should_fill(&self, radial: usize, gate: usize) -> bool;
    fn fill(&mut self, radial: usize, gate: usize);
    fn radial_max(&self) -> usize;
    fn gate_max(&self) -> usize;
}
