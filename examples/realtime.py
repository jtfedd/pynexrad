from time import sleep
from typing import Dict, List, Set
import pynexrad

site = "KDMX"


def get_chunk(
    chunk_cache: Dict[int, List[pynexrad.PyChunk]], 
    chunk_id: pynexrad.PyChunkIdentifier,
) -> None:
    if chunk_id.volume not in chunk_cache:
        chunk_cache[chunk_id.volume] = []

    chunk = pynexrad.download_chunk(chunk_id)
    print("Downloaded", chunk_id.name)

    chunk_cache[chunk_id.volume].append(chunk)


latest_volume = pynexrad.get_latest_volume(site)
print("LATEST VOLUME", latest_volume)

chunks = pynexrad.list_chunks_in_volume(site, latest_volume)
last_chunk = chunks[-1]

chunk_id_cache = {}
for chunk in chunks:
    chunk_id_cache[chunk.name] = chunk

chunk_cache: Dict[int, List[pynexrad.PyChunk]] = {}
for chunk_id in chunks:
    get_chunk(chunk_cache, chunk_id)

while True:
    print("sleeping...")
    sleep(5)
    print()
    print("POLLING")
    print("LAST CHUNK", last_chunk.name)

    dirty_volumes: Set[int] = set()

    chunks = pynexrad.list_chunks_in_volume(site, latest_volume)
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
