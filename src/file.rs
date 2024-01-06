use nexrad::file::FileMetadata;
use pyo3::prelude::*;
use chrono::NaiveDate;

#[derive(Clone)]
#[pyclass(name = "FileMetadata")]
pub struct PyFileMetadata {
    site: String,
    date: NaiveDate,
    identifier: String,
}

#[pymethods]
impl PyFileMetadata {
    #[new]
    pub fn init(site: String, year: i32, month: u32, day: u32, identifier: String) -> Self {
        let date = NaiveDate::from_ymd_opt(year, month, day)
            .expect(&format!("date is valid"));

        Self { site, date, identifier }
    }
}

impl PyFileMetadata {
    pub fn to_nexrad_file_metadata(&self) -> FileMetadata {
        return FileMetadata::new(self.site.clone(), self.date, self.identifier.clone())
    }
}