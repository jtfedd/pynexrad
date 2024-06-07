use crate::model::sweep::Sweep;
use nexrad::model::DataFile;

pub struct Volume {
    pub sweeps: Vec<Sweep>,
}

impl Volume {
    pub(crate) fn new(file: &DataFile) -> Self {
        let mut result_sweeps: Vec<Sweep> = Vec::new();

        let mut sweeps: Vec<_> = file.elevation_scans().iter().collect();
        sweeps.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        for sweep in sweeps {
            result_sweeps.push(Sweep::new(sweep.1));
        }

        Self {
            sweeps: result_sweeps,
        }
    }
}
