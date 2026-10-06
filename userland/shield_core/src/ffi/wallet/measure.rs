//! The measurement entries, shipped in the app since the product's gate is a proof
//! timed on a real phone in someone's hand, not a test harness on a desk.

use super::Wallet;
use crate::bench::{bench_launch, card, BenchReport};
use crate::error::WalletError;
use crate::ffi::CancelToken;
use std::sync::Arc;

impl Wallet {
    /// Prove one real transfer and report its timings, cancellable at the next phase.
    pub fn bench_proof(&self, cancel: Arc<CancelToken>) -> Result<BenchReport, WalletError> {
        let dir =
            self.paths().measurement.parent().map(std::path::Path::to_path_buf).unwrap_or_default();
        self.with(|s| bench_launch(s.account(), &dir, cancel.token()))?
    }

    /// Write a measured run's card beside the wallet and return the path. It holds
    /// timings, sizes and the instance shape, never a key, address, amount or note.
    /// `note` is the shell's word on where it ran, which the core cannot know.
    pub fn save_measurement(
        &self,
        report: BenchReport,
        note: String,
    ) -> Result<String, WalletError> {
        let text = card(&report, &note);
        if text.is_empty() {
            return Err(WalletError::Store { source: crate::error::StoreError::Io });
        }
        let path = &self.paths().measurement;
        std::fs::write(path, text)
            .map_err(|_| WalletError::Store { source: crate::error::StoreError::Io })?;
        Ok(path.to_string_lossy().into_owned())
    }
}
