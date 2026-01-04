use pyo3::pyclass;

use super::py_chunk_identifier::PyChunkIdentifier;

#[pyclass]
#[derive(Clone)]
pub struct PyChunk {
    #[pyo3(get)]
    pub chunk_identifier: PyChunkIdentifier,
    #[pyo3(get)]
    pub data: Vec<u8>,
}

impl PyChunk {
    pub(crate) fn new(chunk_identifier: PyChunkIdentifier, data: Vec<u8>) -> Self {
        Self {
            chunk_identifier,
            data,
        }
    }
}
