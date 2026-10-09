//! Reaching the live session. The lock is held for the whole call, so a scan and a proof cannot
//! interleave on one store, and a poisoned lock reads as locked, never as unchecked state.

use super::Wallet;
use crate::custody::HardwareGuard;
use crate::error::{CustodyError, WalletError};
use crate::wallet::{Paths, Session};
use std::sync::atomic::Ordering;
use std::sync::MutexGuard;

impl Wallet {
    /// Run a read only closure against the live session.
    pub(super) fn with<T>(&self, f: impl FnOnce(&Session) -> T) -> Result<T, WalletError> {
        let held = self.held()?;
        let session = held.as_ref().ok_or(WalletError::Custody { source: CustodyError::Locked })?;
        Ok(f(session))
    }

    /// Run a mutating closure against the live session.
    pub(super) fn with_mut<T>(
        &self,
        f: impl FnOnce(&mut Session) -> Result<T, WalletError>,
    ) -> Result<T, WalletError> {
        let mut held = self.held()?;
        let session = held.as_mut().ok_or(WalletError::Custody { source: CustodyError::Locked })?;
        f(session)
    }
}

impl Wallet {
    /// The live session, or the locked error.
    pub(super) fn held(&self) -> Result<MutexGuard<'_, Option<Session>>, WalletError> {
        self.session.lock().map_err(|_| WalletError::Custody { source: CustodyError::Locked })
    }

    pub(super) fn paths(&self) -> &Paths {
        &self.paths
    }

    pub(super) fn guard(&self) -> &dyn HardwareGuard {
        self.guard.as_ref()
    }

    /// Forget any send waiting for a yes, whenever the unlocked account changes.
    pub(super) fn drop_pending(&self) {
        if let Ok(mut slot) = self.pending.lock() {
            *slot = None;
        }
        if let Ok(mut slot) = self.batch.lock() {
            *slot = None;
        }
    }

    pub(super) fn follow(&self, index: u32) {
        self.active.store(index, Ordering::Release);
    }

    /// A fresh review id, never zero, which a refused review carries.
    pub(super) fn next_review_id(&self) -> u64 {
        self.reviews.fetch_add(1, Ordering::Relaxed).saturating_add(1)
    }
}
