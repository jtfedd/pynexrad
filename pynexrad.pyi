from typing import List


class PySweep:
    """
    PySweep represents a sweep at a single elevation
    """
    elevation: float

    az_first: float
    az_step: float
    az_count: int

    range_first: float
    range_step: float
    range_count: int

    start_time: int
    end_time: int

    data: bytes


class PyLevel2File:
    """
    PyLevel2File contains an entire volume scan
    """

    reflectivity: List[PySweep]
    velocity: List[PySweep]


def list_records(site: str, year: int, month: int, day: int) -> List[str]:
    """
    list_records downloads all available records for the given parameters
    """


def download_nexrad_file(id: str) -> PyLevel2File:
    """
    download_nexrad_file downloads the volume file for the given key
    """
