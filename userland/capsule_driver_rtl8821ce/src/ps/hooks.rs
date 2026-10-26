// NONOS Operating System (AGPL-3.0-or-later)

//! What the proofs reach into power save by; none of it is built into the
//! capsule.

use super::state::Ps;

impl Ps {
    /// For the proofs: power save on for this one, the switch left as it is.
    pub fn force_on(&mut self) {
        self.forced = true;
    }

    /// For the proofs: as if the firmware had been told of the association.
    pub fn assume_ready(&mut self) {
        self.ready = true;
    }

    /// For the proofs: whether the firmware was put to sleep.
    pub fn asleep(&self) -> bool {
        self.asleep
    }
}
