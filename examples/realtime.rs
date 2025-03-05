use std::time::Duration;

use nexrad_data::aws::realtime::{ChunkType, VolumeIndex};
use nexrad_data::aws::realtime::{get_latest_volume, list_chunks_in_volume};
use tokio::time::sleep;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let site = "KDMX";

    let mut latest_volume = get_latest_volume(site).await.ok().unwrap().volume.unwrap();
    dbg!(latest_volume);

    let chunks = list_chunks_in_volume(site, latest_volume, 1000).await.ok().unwrap();

    let mut chunk_cache = HashMap::new();
    for chunk in chunks {
        chunk_cache.insert(chunk.name().to_string(), chunk);
    }

    let mut next_poll = std::time::SystemTime::now() + Duration::from_secs(5);

    loop {
        let now = std::time::SystemTime::now();
        if now < next_poll {
            sleep(next_poll.duration_since(now).unwrap()).await;
        }
        next_poll = std::time::SystemTime::now() + Duration::from_secs(5);

        let now = std::time::SystemTime::now();
        let datetime: chrono::DateTime<chrono::Utc> = now.into();
        dbg!("POLLING", datetime.to_rfc3339());

        let chunks = list_chunks_in_volume(site, latest_volume, 1000).await.ok().unwrap();
        for chunk in chunks {
            if !chunk_cache.contains_key(chunk.name()) {
                chunk_cache.insert(chunk.name().to_string(), chunk.clone());
                dbg!(chunk.name(), chunk.date_time().unwrap());
                next_poll = std::time::SystemTime::from(chunk.date_time().unwrap()) + Duration::from_secs(5);
            }

            if chunk.chunk_type().unwrap() == ChunkType::End {
                let mut next_volume = latest_volume.as_number() + 1;
                if next_volume == 1000 {
                    next_volume = 1;
                }
                latest_volume = VolumeIndex::new(next_volume);
            }
        }
    }
}
