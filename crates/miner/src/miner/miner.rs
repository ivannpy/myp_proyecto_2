use crate::miner::options::MinerOptions;
use std::path::PathBuf;

pub struct Miner {
    root_path: PathBuf,
    options: MinerOptions,
}
