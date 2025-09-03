use reqwest::{Error, blocking::get as blocking_reqwest_get};
use std::{fs::File, io::{Cursor, Write}, path::PathBuf, sync::{Arc, atomic::{AtomicU32, AtomicBool, Ordering}}};
use zip::read::ZipArchive;
use futures::StreamExt;

use crate::{MiniGameCollection, METADATA};

pub fn metadata() -> Result<MiniGameCollection, Error> {
    Ok(blocking_reqwest_get(METADATA)?.json()?)
}

pub fn install(
    is_downloading: Arc<AtomicBool>,
    progress: Arc<AtomicU32>,
    path: String,
    url: String,
) {
    is_downloading.store(true, Ordering::Relaxed);
	
    let path = PathBuf::from(&path); 
	let mut buffer: Vec<u8> = Vec::new();
    let mut downloaded_size: f64 = 0.0;
    
    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .unwrap();

        rt.block_on(async {
            let client = reqwest::Client::new();
            let response = client.get(&url).send().await.unwrap();

            let total_size = response
                .headers()
                .get(reqwest::header::CONTENT_LENGTH)
                .and_then(|header| header.to_str().ok())
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(0);

            let mut stream = response.bytes_stream();

            while let Some(chunk) = stream.next().await {
                match chunk {
                    Ok(data) => {
                        downloaded_size += data.len() as f64;
                        buffer.extend_from_slice(&data);
                        progress.store(
                            ((downloaded_size / total_size as f64) * 100.0) as u32,
                            Ordering::Relaxed,
                        );
                    }
                    Err(_) => {
                        break;
                    }
                }
            }

            let cursor = Cursor::new(buffer);
            let mut archive = ZipArchive::new(cursor).unwrap();
            archive.extract(path).unwrap();

            is_downloading.store(false, Ordering::Relaxed);
        });
    });
}
