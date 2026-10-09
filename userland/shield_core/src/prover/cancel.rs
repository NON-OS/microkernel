//! The cancellation flag a shell holds while a proof runs, and how far the proof has come.

use nox_prover::Phase;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

/// Cooperative cancellation, checked at stage boundaries in `run`, and a cancelled proof is
/// dropped. No stage is cut part way, since killing a prover mid allocation corrupts the store.
#[derive(Clone, Default)]
pub struct Cancel {
    flag: Arc<AtomicBool>,
    /// The last phase the prover finished and the fraction of the proof done then.
    progress: Arc<Mutex<Option<(Phase, f32)>>>,
}

impl Cancel {
    /// A fresh token, not yet cancelled.
    pub fn new() -> Cancel {
        Cancel::default()
    }

    /// Ask for cancellation. Safe to call from any thread, including the UI one.
    pub fn cancel(&self) {
        self.flag.store(true, Ordering::SeqCst);
    }

    /// The flag itself, for a prover that polls it between phases.
    pub(crate) fn flag(&self) -> &AtomicBool {
        &self.flag
    }

    /// Whether cancellation has been asked for.
    pub fn cancelled(&self) -> bool {
        self.flag.load(Ordering::SeqCst)
    }

    /// The prover's progress callback: the phase just finished and the fraction done.
    pub(crate) fn report(&self, phase: Phase, fraction: f32) {
        if let Ok(mut p) = self.progress.lock() {
            *p = Some((phase, fraction));
        }
    }

    /// Forget the progress of an earlier attempt, before a proof starts again.
    pub(crate) fn restart(&self) {
        if let Ok(mut p) = self.progress.lock() {
            *p = None;
        }
    }

    /// The last phase the running proof finished and the fraction done, once it has finished one.
    pub fn progress(&self) -> Option<(Phase, f32)> {
        self.progress.lock().ok().and_then(|p| *p)
    }
}
