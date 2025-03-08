from typing import List


class PySweep:
    elevation: float

    az_first: float
    az_step: float
    az_count: int

    range_first: float
    range_step: float
    range_count: int

    start_time: int
    end_time: int

    date: bytes


class PyLevel2File:
    reflectivity: List[PySweep]
    velocity: List[PySweep]


def list_records(site: str, year: int, month: int, day: int) -> List[str]:
    pass


def download_nexrad_file(id: str) -> PyLevel2File:
    pass
