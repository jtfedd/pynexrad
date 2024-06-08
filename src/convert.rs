use nexrad::model::DataFile;

use crate::{filter::{despeckle::despeckle, velocity_ref_threshold::apply_reflectivity_threshold}, model::volume::Volume, pymodel::py_level2_file::PyLevel2File};

pub fn convert_nexrad_file(data_file: &DataFile) -> PyLevel2File {
    let mut volume = Volume::new(data_file);

    apply_reflectivity_threshold(&mut volume, -5.0);
    despeckle(&mut volume, 50);

    PyLevel2File::new(volume)
}
