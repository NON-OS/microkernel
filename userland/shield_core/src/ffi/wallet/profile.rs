//! The phase profile: one pinned proof timed phase by phase on this phone.

use super::Wallet;
use crate::bench::{profile, ProofProfile};
use crate::error::WalletError;
use crate::ffi::CancelToken;
use std::path::Path;
use std::sync::Arc;

impl Wallet {
    /// Prove the pinned `transfer-eth` vector on `threads` threads, 0 for every core, timing each
    /// phase. It needs no account, and builds the periodic cache first when the folder holds none.
    pub fn profile_proof(
        &self,
        threads: u32,
        cancel: Arc<CancelToken>,
    ) -> Result<ProofProfile, WalletError> {
        let dir = self.paths().measurement.parent().map(Path::to_path_buf).unwrap_or_default();
        profile(&dir, threads, cancel.token())
    }
}
