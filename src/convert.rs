use nexrad::model::DataFile;

use crate::{model::volume::Volume, pymodel::py_level2_file::PyLevel2File};

pub fn convert_nexrad_file(data_file: &DataFile) -> PyLevel2File {
    let reflectivity = Volume::new(data_file, "ref");
    let velocity = Volume::new(data_file, "vel");

    PyLevel2File::new(reflectivity, velocity)
}
