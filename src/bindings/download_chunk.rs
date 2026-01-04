use nexrad_data::aws::realtime::{
    download_chunk as nexrad_download_chunk, ChunkIdentifier, VolumeIndex,
};
use pyo3::{pyfunction, PyResult, Python};

use crate::pymodel::{py_chunk::PyChunk, py_chunk_identifier::PyChunkIdentifier};

#[pyfunction]
pub fn download_chunk(py: Python, chunk_identifier: PyChunkIdentifier) -> PyResult<PyChunk> {
    let result = py.allow_threads(move || download_chunk_impl(chunk_identifier));

    Ok(result)
}

fn download_chunk_impl(chunk_identifier: PyChunkIdentifier) -> PyChunk {
    let nexrad_chunk_identifier = ChunkIdentifier::new(
        chunk_identifier.site.clone(),
        VolumeIndex::new(chunk_identifier.volume as usize),
        chunk_identifier.name.clone(),
        None,
    );

    let rt = tokio::runtime::Runtime::new().unwrap();

    let (_, chunk) = rt
        .block_on(async {
            nexrad_download_chunk(&chunk_identifier.site, &nexrad_chunk_identifier).await
        })
        .expect("Should find latest volume");

    let py_chunk = PyChunk::new(chunk_identifier, chunk.data().to_vec());

    py_chunk
}
