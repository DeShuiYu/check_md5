use std::fs::File;
use std::path::{PathBuf};
use std::{io, time};
use clap::Parser;
use md5::Digest;
use rayon::prelude::*;
use walkdir::WalkDir;
use hex;

#[derive(Parser)]
struct Cli {
    #[arg(short = 'f', long = "from_dir", default_value = "/Users/dsy/Desktop")]
    from_dir: Option<PathBuf>,
}

fn calc_md5(file_path: &PathBuf) -> String {
    let mut file = File::open(file_path).expect("Failed to read file");
    let mut hasher = md5::Md5::new();
    let _ = io::copy(&mut file, &mut hasher);
    return hex::encode(hasher.finalize());
}

fn main() {
    let st = time::Instant::now();
    let cli = Cli::parse();
    let entries: Vec<_> = WalkDir::new(cli.from_dir.unwrap())
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .collect();
    entries.
        par_iter()
        .for_each(|entry| {
            println!("{}\t{}",entry.path().to_string_lossy(),calc_md5(&entry.path().to_path_buf()));
    });
    println!("elapsed time:{}(s)", st.elapsed().as_secs());
}
