// Copyright (c) Meta Platforms, Inc. and affiliates.
// All rights reserved.

// This source code is licensed under the license found in the
// LICENSE file in the root directory of this source tree.

use anyhow::Context;
use clap::Parser;
use std::path::PathBuf;
mod constants;

#[derive(Parser)]
#[command(version, about, arg_required_else_help = true)]
struct Args {
    /// Filepath to cache ID
    #[arg(short, long, default_value = constants::DEFAULT_CACHE_FILEPATH)]
    cache_filepath: PathBuf,

    /// Print ID to stdout
    #[arg(short, long)]
    stdout: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let gpu_node_id =
        gni_lib::get_gpu_node_id(Some(&args.cache_filepath)).context("Failed to get id")?;

    if args.stdout {
        println!("{}", gpu_node_id);
    }
    Ok(())
}
