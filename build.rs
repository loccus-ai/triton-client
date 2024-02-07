use prost_build::Config;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use tonic_build::configure;
use walkdir::WalkDir;

fn main() {
    let pb_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/common/protobuf");

    let protobuf_paths = get_protobuf_paths(pb_dir);

    for path in &protobuf_paths {
        println!("cargo:rerun-if-changed={}", path.display());
    }

    let mut config = Config::new();
    config.bytes(["."]);
    configure()
        .build_server(true)
        .compile_with_config(config, &protobuf_paths, &[pb_dir])
        .unwrap();
}

fn get_protobuf_paths<P: AsRef<Path>>(directory: P) -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = vec![];
    for entry in WalkDir::new(directory) {
        let path = entry.unwrap().into_path();
        if path.extension() == Some(OsStr::new("proto")) {
            paths.push(path.to_path_buf());
        }
    }
    paths
}
