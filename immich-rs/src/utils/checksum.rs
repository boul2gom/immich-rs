#[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
use std::path::Path;

use sha1::{Digest, Sha1};

#[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
use crate::error::Result;

/// Calculate the SHA-1 checksum of a slice of bytes.
#[must_use]
pub fn calculate_bytes_checksum(data: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

/// Calculate the SHA-1 checksum of a file efficiently by reading it in chunks on a blocking worker thread.
///
/// # Errors
///
/// Returns an error if the file cannot be opened or read.
#[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
pub async fn calculate_file_checksum<P: AsRef<Path>>(path: P) -> Result<String> {
    let path = path.as_ref().to_owned();
    tokio::task::spawn_blocking(move || {
        use std::io::Read;
        let mut file = std::fs::File::open(path)?;
        let mut hasher = Sha1::new();
        let mut buffer = vec![0; 65536];

        loop {
            let n = file.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
        }

        let result = hasher.finalize();
        Ok(hex::encode(result))
    })
    .await
    .map_err(|e| {
        if e.is_panic() {
            std::panic::resume_unwind(e.into_panic());
        }
        crate::Error::Io(std::io::Error::other(e))
    })?
}
