use nexrad_data::{aws::realtime::Chunk, volume::Record};
use pyo3::{pyfunction, PyResult, Python};

use crate::pymodel::{py_chunk::PyChunk, py_level2_file::PyLevel2File};

use super::convert::convert_nexrad_file;

#[pyfunction]
pub fn convert_chunks(py: Python, chunks: Vec<PyChunk>) -> PyResult<PyLevel2File> {
    let result = py.allow_threads(move || convert_chunks_impl(chunks));

    Ok(result)
}

fn convert_chunks_impl(chunks: Vec<PyChunk>) -> PyLevel2File {
    let nexrad_chunks: Vec<_> = chunks
        .iter()
        .map(|chunk| Chunk::new(chunk.data.clone()).expect("Can construct chunk from data"))
        .collect();

    let mut records: Vec<Record> = Vec::new();

    for chunk in nexrad_chunks {
        match chunk {
            Chunk::Start(start_chunk) => {
                for record in start_chunk.records() {
                    records.push(Record::new(record.data().to_vec()));
                }
            }
            Chunk::IntermediateOrEnd(mid_chunk) => records.push(mid_chunk),
        };

        // records.extend(chunk_records);
    }

    convert_nexrad_file(records)
}
