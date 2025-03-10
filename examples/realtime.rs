use std::time::Duration;

use nexrad_data::aws::realtime::{download_chunk, Chunk, ChunkIdentifier, ChunkType, VolumeIndex};
use nexrad_data::aws::realtime::{get_latest_volume, list_chunks_in_volume};
use nexrad_data::volume::Record;
use pynexrad::convert::convert_nexrad_file;
use pynexrad::pymodel::py_level2_file::PyLevel2File;
use std::collections::{HashMap, HashSet};
use tokio::time::sleep;

async fn get_chunk<'a>(
    chunk_cache: &mut HashMap<usize, Vec<Chunk<'a>>>,
    chunk_id: ChunkIdentifier,
    site: &str,
) {
    if !chunk_cache.contains_key(&chunk_id.volume().as_number()) {
        chunk_cache.insert(chunk_id.volume().as_number(), Vec::new());
    }

    let (_, chunk) = download_chunk(site, &chunk_id).await.ok().unwrap();
    println!("Downloaded {}", chunk_id.name());

    chunk_cache
        .get_mut(&chunk_id.volume().as_number())
        .unwrap()
        .push(chunk);
}

fn get_records<'a>(chunks: &'a Vec<Chunk<'a>>) -> Vec<Record<'a>> {
    let mut records: Vec<Record> = Vec::new();

    for chunk in chunks {
        let chunk_records: Vec<Record> = match chunk {
            Chunk::Start(start_chunk) => start_chunk.records().iter().cloned().collect(),
            Chunk::IntermediateOrEnd(mid_chunk) => vec![mid_chunk.clone()],
        };

        records.extend(chunk_records);
    }

    records
}

#[tokio::main]
async fn main() {
    let site = "KDMX";

    let mut latest_volume = get_latest_volume(site).await.ok().unwrap().volume.unwrap();
    println!("LATEST VOLUME: {}", latest_volume.as_number());

    let chunks = list_chunks_in_volume(site, latest_volume, 1000)
        .await
        .ok()
        .unwrap();
    let mut last_chunk = chunks.last().unwrap().clone();

    let mut chunk_id_cache: HashMap<String, ChunkIdentifier> = HashMap::new();
    for chunk in chunks.iter() {
        chunk_id_cache.insert(chunk.name().to_string(), chunk.clone());
    }

    let mut chunk_cache: HashMap<usize, Vec<Chunk>> = HashMap::new();
    for chunk_id in chunks {
        get_chunk(&mut chunk_cache, chunk_id, site).await;
    }

    let mut volume_cache: HashMap<usize, PyLevel2File> = HashMap::new();
    volume_cache.insert(
        latest_volume.as_number(),
        convert_nexrad_file(get_records(
            chunk_cache.get(&latest_volume.as_number()).unwrap(),
        )),
    );

    loop {
        println!("sleeping...");
        sleep(Duration::from_secs(5)).await;
        println!();
        println!("POLLING");
        println!("LAST CHUNK {}", last_chunk.name());

        let mut dirty_volumes: HashSet<usize> = HashSet::new();

        let chunks = list_chunks_in_volume(site, latest_volume, 1000)
            .await
            .ok()
            .unwrap();
        for chunk_id in chunks {
            if !chunk_id_cache.contains_key(chunk_id.name()) {
                dirty_volumes.insert(chunk_id.volume().as_number());

                chunk_id_cache.insert(chunk_id.name().to_string(), chunk_id.clone());
                get_chunk(&mut chunk_cache, chunk_id.clone(), site).await;

                let chunk_clone = chunk_id.clone();
                last_chunk = chunk_clone;

                if last_chunk.chunk_type().unwrap() == ChunkType::End {
                    let mut next_volume = latest_volume.as_number() + 1;
                    if next_volume == 1000 {
                        next_volume = 1;
                    }
                    latest_volume = VolumeIndex::new(next_volume);
                }
            }
        }

        for volume_id in dirty_volumes {
            println!("Recalculate volume {}", volume_id);

            let updated_volume =
                convert_nexrad_file(get_records(chunk_cache.get(&volume_id).unwrap()));

            println!("Updated Volume");
            println!("REF {}", updated_volume.reflectivity.len());
            println!("VEL {}", updated_volume.velocity.len());

            volume_cache.insert(volume_id, updated_volume);
        }
    }
}
