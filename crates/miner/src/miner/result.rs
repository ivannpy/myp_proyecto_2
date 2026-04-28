use crate::metadata::track::TrackMetadata;
use crate::miner::errors::MinerError;

#[derive(Debug)]
pub struct MinerResult {
    pub tracks: Vec<TrackMetadata>,
    pub errors: Vec<MinerError>,
}
