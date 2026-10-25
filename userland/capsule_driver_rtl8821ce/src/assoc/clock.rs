// NONOS Operating System (AGPL-3.0-or-later)

//! The join's time, and its pause while the radio has nothing for it.

/// What a join reads the time from, and how it waits when nothing was heard.
pub trait JoinClock {
    fn now_ms(&mut self) -> u64;
    /// Give the processor up for a moment: the radio had nothing to read.
    fn idle(&mut self);
}

/// A bare reading of the time, as the proofs drive a join: no pause.
impl<F: FnMut() -> u64> JoinClock for F {
    fn now_ms(&mut self) -> u64 {
        self()
    }
    fn idle(&mut self) {}
}
