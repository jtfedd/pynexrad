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


class PyChunkIdentifier:
    """
    PyChunkIdentifier identifies a particular chunk
    """

    site: str
    volume: int
    name: str


class PyChunk:
    """
    PyChunk contains the date for a chunk
    """

    chunk_identifier: PyChunkIdentifier
    data: bytes


def get_latest_volume(site: str) -> int:
    """
    get_latest_volume finds the latest volume with data for a given site
    """


def list_chunks_in_volume(
    site: str,
    volume_id: int
) -> List[PyChunkIdentifier]:
    """
    list_chunks_in_volume lists the chunks currently available for
    the given site and volume id
    """


def download_chunk(chunk_identifier: PyChunkIdentifier) -> PyChunk:
    """
    download_chunk downloads the specified chunk
    """


def convert_chunks(chunks: List[PyChunk]) -> PyLevel2File:
    """
    convert_chunks converts the list of chunks (all from the same volume)
    into a volume file
    """


def list_records(site: str, year: int, month: int, day: int) -> List[str]:
    """
    list_records downloads all available records for the given parameters
    """


def download_nexrad_file(id: str) -> PyLevel2File:
    """
    download_nexrad_file downloads the volume file for the given key
    """
