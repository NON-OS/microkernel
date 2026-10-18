// NONOS Operating System (AGPL-3.0-or-later)

//! A scan's answer that counts how often it is dropped, for the hand-off
//! proofs.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use crate::handoff::Handoff;

/// A scan's answer, shaped like the panel's: a count and the networks'
/// signals, dropped once.
#[derive(Debug)]
pub(crate) struct Scanned {
    pub(crate) signals: Vec<i8>,
    pub(crate) drops: Arc<AtomicUsize>,
}

impl PartialEq for Scanned {
    fn eq(&self, other: &Self) -> bool {
        self.signals == other.signals
    }
}

impl Drop for Scanned {
    fn drop(&mut self) {
        self.drops.fetch_add(1, Ordering::SeqCst);
    }
}

pub(crate) fn scanned(signals: &[i8], drops: &Arc<AtomicUsize>) -> Scanned {
    Scanned { signals: signals.to_vec(), drops: drops.clone() }
}

pub(crate) fn leaked<T>() -> &'static Handoff<T> {
    Box::leak(Box::new(Handoff::new()))
}
