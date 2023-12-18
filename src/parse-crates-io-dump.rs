use csv;
use clap::Parser;
use serde::Deserialize;
use tempfile;

use std::process::Command;
use std::io::{Read, BufRead, BufReader};
use std::{fs, path::PathBuf};

#[derive(Debug,Deserialize)]
struct CrateMetadata {
    created_at: String,
    //description: String, // skip
    //documentation: String, // skip
    // downloads: u32,
    homepage: String,
    id: String,
    // max_features, // skip
    // max_upload_size: u32,
    name: String,
    // readme,
    repository: String,
    updated_at: String,
}

#[derive(Parser, Debug)]
struct Args {

    /// File to parse
    #[arg(short, long, value_hint(clap::ValueHint::FilePath))]
    file: PathBuf,

    /// Directory to use for temporary use
    #[arg(short, long, value_hint(clap::ValueHint::FilePath))]
    working_directory: Option<PathBuf>,
}



fn main() {
    let args = Args::parse();

    let working_directory = args.working_directory.unwrap_or_else(|| {
        tempfile::Builder::new().prefix("heatshield-").tempdir().unwrap().into_path()
    });

    let mut reader = csv::Reader::from_path(args.file).unwrap();
    let crates = reader.deserialize::<CrateMetadata>().take(10);
    for meta in crates {
        // println!("{:?}")
        match meta {
            Ok(meta) => {
                if meta.repository.is_empty() {
                    continue;
                }

                println!("{}", meta.repository);
                println!("{:?}", working_directory.as_os_str());

                Command::new("cd")
                    .args([working_directory.as_os_str()])
                    .spawn()
                    .unwrap();

                let git_clone = Command::new("git")
                    .args(["clone", &meta.repository, "working"])
                    .output()
                    .unwrap();

                for line in git_clone.stdout.lines().flatten() {
                    println!("git-clone: {line}")
                }

                Command::new("cd")
                    .args(["working"])
                    .spawn()
                    .unwrap();

                let git_branch = Command::new("git")
                    .args(["branch", "--show-current"])
                    .output()
                    .unwrap();

                for line in git_branch.stdout.lines().flatten() {
                    println!("git-branch: {line}")
                }

                let rev_parse = Command::new("git")
                    .args(["rev-parse", "HEAD"])
                    .output()
                    .unwrap();

                for line in rev_parse.stdout.lines().flatten() {
                    println!("git-branch: {line}")
                }
            },
            Err(err) => {
                println!("{err:?}");
            },
        }

        // break;
    }

    Command::new("rm")
        .args(["-rf", "/tmp/heatshield-*"])
        .spawn()
        .expect("cleaning up");
}