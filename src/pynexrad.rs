use pyo3::prelude::*;

use crate::bindings::download_chunk::download_chunk;
use crate::bindings::download_nexrad_file::download_nexrad_file;
use crate::bindings::get_latest_volume::get_latest_volume;
use crate::bindings::list_chunks_in_volume::list_chunks_in_volume;
use crate::bindings::list_records::list_records;
use crate::bindings::convert_chunks::convert_chunks;
use crate::pymodel::py_chunk::PyChunk;
use crate::pymodel::py_chunk_identifier::PyChunkIdentifier;
use crate::pymodel::py_level2_file::PyLevel2File;
use crate::pymodel::py_sweep::PySweep;

#[pymodule]
fn pynexrad(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(list_records, m)?)?;
    m.add_function(wrap_pyfunction!(download_nexrad_file, m)?)?;

    m.add_class::<PyLevel2File>()?;
    m.add_class::<PySweep>()?;

    m.add_function(wrap_pyfunction!(get_latest_volume, m)?)?;
    m.add_function(wrap_pyfunction!(list_chunks_in_volume, m)?)?;
    m.add_function(wrap_pyfunction!(download_chunk, m)?)?;
    m.add_function(wrap_pyfunction!(convert_chunks, m)?)?;

    m.add_class::<PyChunkIdentifier>()?;
    m.add_class::<PyChunk>()?;

    Ok(())
}
