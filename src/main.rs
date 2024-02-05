use std::{
    fs, io,
    path::{Path, PathBuf},
};

use clap::Parser;

fn main() {
    parse_cli();
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(
        short = 'd',
        long = "dir",
        help = "The directory to search",
        default_value = "."
    )]
    directory: PathBuf,
    #[arg(
        short = 'o',
        long = "output",
        help = "The directory to copy the files to",
        default_value = "."
    )]
    output: PathBuf,
    #[arg(
        short = 'l',
        long = "len",
        help = "The size of each batch",
        default_value = "100"
    )]
    batch_size: usize,
}

fn parse_cli() {
    let args = Args::parse();
    let files = find_files(&args.directory);

    for batch in files.chunks(args.batch_size) {
        println!("Processing {} files", batch.len());
    }

    let chunks = chunk_dir(&args.directory, args.batch_size);
    for (i, chunk) in chunks.iter().enumerate() {
        copy_files(&chunk, &args.output, i).expect("copy_files call failed");
    }
}

fn find_files(path: &Path) -> Vec<PathBuf> {
    if !path.is_dir() {
        panic!("{} is not a directory", path.display());
    }

    let mut files = Vec::new();

    path.read_dir()
        .expect("read_dir call failed")
        .filter_map(|entry| entry.ok())
        .for_each(|entry| {
            let path = entry.path();

            if path.is_file() {
                files.push(path);
            }
        });
    files
}

fn chunk_dir(path: &Path, chunk_size: usize) -> Vec<Vec<PathBuf>> {
    let files = find_files(path);
    files.chunks(chunk_size).map(|c| c.to_vec()).collect()
}

fn copy_files(files: &[PathBuf], to: &Path, chunk_num: usize) -> io::Result<()> {
    for file in files {
        let parent = file.parent().ok_or(io::ErrorKind::NotFound)?;
        let dest = parent.join(format!("{}_{}", to.display(), chunk_num));
        fs::create_dir_all(&dest)?;
        let file_name = file.file_name().ok_or(io::ErrorKind::NotFound)?;
        let dest = dest.join(file_name);
        fs::copy(file, dest)?;
    }
    Ok(())
}
