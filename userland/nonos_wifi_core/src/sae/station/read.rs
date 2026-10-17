// NONOS Operating System (AGPL-3.0-or-later)

//! What an exchange shows: its state, why it last discarded, its keys.

use super::state::SaeStation;
use super::types::{SaeFailure, SaeState};

impl SaeStation {
    pub fn state(&self) -> SaeState {
        self.state
    }

    /// Why the last frame was discarded, while the exchange runs.
    pub fn failure(&self) -> Option<SaeFailure> {
        self.failure
    }

    /// The PMK and PMKID once accepted.
    pub fn pmk(&self) -> Option<([u8; 32], [u8; 16])> {
        (self.state == SaeState::Accepted).then_some((self.keys.pmk, self.keys.pmkid))
    }
}
