// NONOS Operating System (AGPL-3.0-or-later)

//! A transmit ring the card stopped reading. Frames queued while the card's
//! read index stands still for `STALL_MS` mean its DMA engine is wedged; the
//! link resets the rings rather than leave the queue full for good (plan/50
//! C1).

/// How long queued frames may wait with the read index still: the Linux netdev
/// watchdog's transmit timeout.
pub const STALL_MS: u64 = 5000;

/// The read index last seen, and since when it has stood still with frames
/// queued.
#[derive(Clone, Copy, Default)]
pub struct TxWatch {
    rp: u32,
    since_ms: Option<u64>,
}

impl TxWatch {
    /// Note the ring at `now_ms`: whether frames are `pending` and the card's
    /// read index `rp`. True once it has stood still with frames queued for
    /// `STALL_MS`.
    pub fn stalled(&mut self, pending: bool, rp: u32, now_ms: u64) -> bool {
        match self.since_ms {
            Some(since) if pending && rp == self.rp => now_ms.saturating_sub(since) >= STALL_MS,
            _ => {
                *self = Self { rp, since_ms: pending.then_some(now_ms) };
                false
            }
        }
    }
}
