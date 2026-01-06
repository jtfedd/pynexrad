use nexrad_data::aws::realtime::ChunkIdentifier;
use pyo3::pyclass;

#[pyclass]
#[derive(Clone)]
pub struct PyChunkIdentifier {
    #[pyo3(get)]
    pub site: String,
    #[pyo3(get)]
    pub volume: i32,
    #[pyo3(get)]
    pub name: String,
}

impl PyChunkIdentifier {
    pub(crate) fn new(chunk_identifier: &ChunkIdentifier) -> Self {
        Self {
            site: chunk_identifier.site().to_string(),
            volume: chunk_identifier.volume().as_number() as i32,
            name: chunk_identifier.name().to_string(),
        }
    }
}
