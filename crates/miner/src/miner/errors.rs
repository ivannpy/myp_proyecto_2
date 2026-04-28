use std::path::PathBuf;

#[derive(Debug)]
pub enum MinerError {
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    Tag {
        path: PathBuf,
        source: id3::Error,
    },
    ReadDir {
        path: PathBuf,
        source: std::io::Error,
    },
}
