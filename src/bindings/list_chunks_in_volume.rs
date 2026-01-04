use nexrad_data::aws::realtime::{list_chunks_in_volume as nexrad_chunks_in_volume, VolumeIndex};
use pyo3::{pyfunction, PyResult, Python};

use crate::pymodel::py_chunk_identifier::PyChunkIdentifier;

#[pyfunction]
pub fn list_chunks_in_volume(
    py: Python,
    site: String,
    volume_id: i32,
) -> PyResult<Vec<PyChunkIdentifier>> {
    let result = py.allow_threads(move || list_chunks_in_volume_impl(site, volume_id));

    Ok(result)
}

fn list_chunks_in_volume_impl(site: String, volume_id: i32) -> Vec<PyChunkIdentifier> {
    let volume_index = VolumeIndex::new(volume_id as usize);

    let rt = tokio::runtime::Runtime::new().unwrap();

    let chunks_in_volume = rt
        .block_on(async { nexrad_chunks_in_volume(&site, volume_index, 1000).await })
        .expect("Should have chunks in the volume");

    let chunk_identifiers = chunks_in_volume
        .iter()
        .map(|chunk| PyChunkIdentifier::new(chunk))
        .collect::<Vec<_>>();

    chunk_identifiers
}
