// NONOS Operating System (AGPL-3.0-or-later)

//! The candidates taken in order, best first, a bounded number of times.

use super::candidate::Candidate;
use super::list::{Candidates, MAX_TRIES};
use crate::mlme::MlmeFailure;

impl Candidates {
    /// The best candidate not yet tried, while tries remain.
    pub fn take_best(&mut self) -> Option<Candidate> {
        if self.tried >= MAX_TRIES {
            return None;
        }
        let best = (0..self.list.len()).max_by_key(|&i| self.list[i].key())?;
        self.tried += 1;
        Some(self.list.swap_remove(best))
    }

    /// Whether any candidate was heard.
    pub fn found(&self) -> bool {
        !self.list.is_empty()
    }

    /// Why the last frame for the network that could not be joined was refused.
    pub fn refusal(&self) -> Option<MlmeFailure> {
        self.refusal
    }
}
