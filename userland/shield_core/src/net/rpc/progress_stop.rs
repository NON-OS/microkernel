//! The stop a read of the pool watches, so a lock never waits on a sync.

use crate::error::NetError;
use std::sync::atomic::{AtomicU64, Ordering};

/// What a read of the pool watches to know it should stop: a count of the stops its wallet
/// was told, and that count when the read started. A read stops at its next request once the
/// count has moved, so a lock never waits minutes on a sync over a mixnet, and one wallet's
/// lock never stops another's read.
#[derive(Clone, Copy)]
pub struct Stop<'a> {
    stops: &'a AtomicU64,
    epoch: u64,
}

/// The count of a read that is never stopped.
static NEVER: AtomicU64 = AtomicU64::new(0);

impl<'a> Stop<'a> {
    /// A read stopped by any later stop counted in `stops`.
    pub fn watching(stops: &'a AtomicU64) -> Stop<'a> {
        Stop { stops, epoch: stops.load(Ordering::SeqCst) }
    }

    /// A read nothing stops, for the callers that hold no wallet.
    pub fn never() -> Stop<'static> {
        Stop::watching(&NEVER)
    }

    /// Whether this read was told to stop since it started.
    pub(crate) fn check(&self) -> Result<(), NetError> {
        if self.stops.load(Ordering::SeqCst) == self.epoch {
            Ok(())
        } else {
            Err(NetError::Stopped)
        }
    }
}
