use clap::Parser;
use std::path::PathBuf;
use anyhow::Context;

#[derive(Parser)]
#[command(version, about, arg_required_else_help = true)]
struct Args {
    /// Filepath to cache ID
    #[arg(short, long, default_value = "/tmp/gni")]
    cache_filepath: PathBuf,

    /// Print ID to stdout
    #[arg(short, long)]
    stdout: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let gpu_node_id = gni::get_gpu_node_id(&args.cache_filepath).context("Failed to get id")?;

    if args.stdout {
        println!("{}", gpu_node_id);
    }
    Ok(())
}