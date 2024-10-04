use std::fs;
use std::path::Path;
use std::path::PathBuf;
use anyhow::{anyhow, Context, Result};

use blake3::Hasher;
use nvml_wrapper::Nvml;

fn blake3_hash_string(input: &str) -> String {
    let mut hasher: Hasher = Hasher::new();
    hasher.update(input.as_bytes());
    let result: blake3::Hash = hasher.finalize();
    result.to_hex().to_string()
}

/// Returns the GPU Node ID as String
pub fn get_gpu_node_id(cache_file_path: &PathBuf) -> Result<String, anyhow::Error> {
    if Path::new(cache_file_path).exists() {
        let contents: String = fs::read_to_string(cache_file_path).context("Failed to read cache file")?;
        return Ok(contents);
    }

    let nvml: Nvml = Nvml::init().context("Failed to init nvml")?;
    let device_count: u32 = nvml.device_count().context("Failed to get nvml device count")?;
    let mut uuids = Vec::new();

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

    fs::write(cache_file_path, &gpu_node_id).context("Failed to write cache")?;

    Ok(gpu_node_id)
}