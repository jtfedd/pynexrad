use nexrad::model::DataFile;
use crate::model::PyLevel2File;

pub fn convert_nexrad_file(file: &DataFile) -> &PyLevel2File {
    return &PyLevel2File::new(reflectivity, velocity)
}