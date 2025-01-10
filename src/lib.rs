use std::fs;
use std::path::Path;
use std::path::PathBuf;
use anyhow::{anyhow, Context, Result};

#[cfg(feature = "c")]
mod c;

#[cfg(feature = "cpp")]
mod cpp;

#[cfg(feature = "python")]
mod python;

mod constants;

use blake3::Hasher;
use nvml_wrapper::Nvml;

fn blake3_hash_string(input: &str) -> String { 
    let mut hasher: Hasher = Hasher::new();
    hasher.update(input.as_bytes());
    let result: blake3::Hash = hasher.finalize();
    result.to_hex().to_string()
}

/// Returns the GPU Node ID as String
pub fn get_gpu_node_id(cache_file_path: Option<&PathBuf>) -> Result<String, anyhow::Error> {
    let default_path: &Path = Path::new(constants::DEFAULT_CACHE_FILEPATH);
    let binding: PathBuf = default_path.to_path_buf();
    let path: &PathBuf = cache_file_path.unwrap_or(&binding);

    if Path::new(path).exists() {
        let contents: String = fs::read_to_string(path).context("Failed to read cache file")?;
        return Ok(contents);
    }

    let nvml: Nvml = Nvml::init().context("Failed to init nvml")?;
    let device_count: u32 = nvml.device_count().context("Failed to get nvml device count")?;
    let mut uuids: Vec<String> = Vec::new();

    for n in 0..device_count {
        let device: nvml_wrapper::Device<'_> = nvml.device_by_index(n).context("Failed to get nvml device by index")?;
        let uuid: String = device.uuid().context("Failed to get device uuid")?;
        uuids.push(uuid);
    }

    if uuids.is_empty() {
        return Err(anyhow!("No GPUs found"));
    }

    // sort the UUIDs to ensure a consistent hash (the node ID should be the same regardless of the order of the GPUs)
    uuids.sort();

    let concatenated_uuids: String = uuids.join("");

    let gpu_node_id: String = blake3_hash_string(&concatenated_uuids);

    fs::write(path, &gpu_node_id).context("Failed to write cache")?;

    Ok(gpu_node_id)
}
