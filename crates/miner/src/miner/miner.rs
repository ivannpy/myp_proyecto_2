use crate::metadata::track::TrackMetadata;
use crate::miner::errors::MinerError;
use crate::miner::options::MinerOptions;
use crate::miner::result::MinerResult;
use id3::{Tag, TagLike};
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

pub struct Miner {
    pub root_path: PathBuf,
    pub options: MinerOptions,
}

impl Miner {
    pub fn mine(&self) -> MinerResult {
        let mut tracks = Vec::new();
        let mut errors = Vec::new();

        let walker = WalkDir::new(&self.root_path)
            .follow_links(self.options.follow_symlinks)
            .max_depth(self.options.max_depth.unwrap_or(usize::MAX));

        for entry in walker {
            match entry {
                Err(e) => {
                    errors.push(MinerError::ReadDir {
                        path: e.path().unwrap_or(Path::new("")).to_owned(),
                        source: e.into_io_error().unwrap(),
                    });
                }
                Ok(entry) => {
                    if !self.is_admitted(&entry) {
                        continue;
                    }

                    match self.extract_metadata(entry.path()) {
                        Ok(track) => tracks.push(track),
                        Err(e) => errors.push(e),
                    }
                }
            }
        }

        MinerResult { tracks, errors }
    }

    fn is_admitted(&self, entry: &DirEntry) -> bool {
        entry.file_type().is_file()
            && entry
                .path()
                .extension()
                .map(|e| e.eq_ignore_ascii_case("mp3") || e.eq_ignore_ascii_case("flac"))
                .unwrap_or(false)
    }

    fn extract_metadata(&self, path: &Path) -> Result<TrackMetadata, MinerError> {
        let tag = Tag::read_from_path(path).map_err(|e| MinerError::Tag {
            path: path.to_owned(),
            source: e,
        })?;

        let _ = fs::metadata(path).map_err(|e| MinerError::Io {
            path: path.to_owned(),
            source: e,
        })?;

        Ok(TrackMetadata {
            path: path.to_owned(),
            title: tag.title().map(str::to_owned),
            artist: tag.artist().map(str::to_owned),
            album: tag.album().map(str::to_owned),
            track_number: tag.track(),
            year: tag.year(),
            genre: tag.genre().map(str::to_owned),
        })
    }
}
