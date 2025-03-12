from time import sleep
from typing import Dict, List, Set
from pynexrad import (
    PyChunk,
    PyChunkIdentifier, 
    download_chunk, 
    list_chunks_in_volume, 
    get_latest_volume,
    convert_chunks,
)

site = "KDMX"


def get_chunk(
    chunk_cache: Dict[int, List[PyChunk]], 
    chunk_id: PyChunkIdentifier,
) -> None:
    if chunk_id.volume not in chunk_cache:
        chunk_cache[chunk_id.volume] = []

    chunk = download_chunk(chunk_id)
    print("Downloaded", chunk_id.name)

    chunk_cache[chunk_id.volume].append(chunk)


latest_volume = get_latest_volume(site)
print("LATEST VOLUME", latest_volume)

chunks = list_chunks_in_volume(site, latest_volume)
last_chunk = chunks[-1]

chunk_id_cache = {}
for chunk in chunks:
    chunk_id_cache[chunk.name] = chunk

chunk_cache: Dict[int, List[PyChunk]] = {}
for chunk_id in chunks:
    get_chunk(chunk_cache, chunk_id)

volume_cache = {}
volume_cache[latest_volume] = convert_chunks(chunk_cache[latest_volume])

while True:
    print("sleeping...")
    sleep(5)
    print()
    print("POLLING")
    print("LAST CHUNK", last_chunk.name)

    dirty_volumes: Set[int] = set()

    chunks = list_chunks_in_volume(site, latest_volume)
    for chunk_id in chunks:
        if chunk_id.name not in chunk_id_cache:
            dirty_volumes.add(chunk_id.volume)

            chunk_id_cache[chunk_id.name] = chunk_id
            get_chunk(chunk_cache, chunk_id)

            last_chunk = chunk_id

            if last_chunk.name.endswith("E"):
                latest_volume = latest_volume + 1
                if latest_volume == 1000:
                    latest_volume = 1

    for volume_id in dirty_volumes:
        print("Recalculate volume", volume_id)

        updated_volume = convert_chunks(chunk_cache[volume_id])

        print("Updated Volume", volume_id)
        print("REF", len(updated_volume.reflectivity))
        print("VEL", len(updated_volume.velocity))

        volume_cache[volume_id] = updated_volume
