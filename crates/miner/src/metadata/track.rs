use std::path::PathBuf;

pub struct TrackMetadata {
    pub path: PathBuf,
    pub title: Option<String>,
    pub artist: Option<String>,
}
