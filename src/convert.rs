use nexrad::model::DataFile;

use crate::dealias_region::region_dealias::dealias_region_based;
use crate::filter::despeckle::despeckle;
use crate::filter::velocity_ref_threshold::apply_reflectivity_threshold;
use crate::model::volume::Volume;
use crate::pymodel::py_level2_file::PyLevel2File;

pub fn convert_nexrad_file(data_file: &DataFile) -> PyLevel2File {
    let mut volume = Volume::new(data_file);

    apply_reflectivity_threshold(&mut volume, -5.0);
    despeckle(&mut volume, 50);
    dealias_region_based(&mut volume, 3, 100, 100, true);

    PyLevel2File::new(volume)
}
