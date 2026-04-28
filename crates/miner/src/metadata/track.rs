use std::path::PathBuf;

#[derive(Debug)]
pub struct TrackMetadata {
    pub path: PathBuf,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub track_number: Option<u32>,
    pub year: Option<i32>,
    pub genre: Option<String>,
}
