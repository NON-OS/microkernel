//! Every view-only account of an unlocked wallet, and where their files live.

use super::Watched;
use std::path::PathBuf;
use zeroize::Zeroizing;

/// Every view-only account of an unlocked wallet, with the key their files are sealed under.
pub(crate) struct Watching {
    pub(super) root: Zeroizing<[u8; 32]>,
    pub(super) file: PathBuf,
    pub(super) notes: PathBuf,
    pub(crate) list: Vec<Watched>,
}
