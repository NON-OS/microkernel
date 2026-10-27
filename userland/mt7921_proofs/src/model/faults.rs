// NONOS Operating System (AGPL-3.0-or-later)

//! What the modelled chip can be told to do wrong, for the proofs of what
//! the driver does then.

#[derive(Default, Clone)]
pub struct Faults {
    /// Every read is all ones and every write lost: off the bus.
    pub dead: bool,
    /// `CLR_OWN` never clears `OWN_SYNC`: the driver never owns the chip.
    pub no_own: bool,
    /// The subsystem reset never reports `INIT_DONE`.
    pub no_reset: bool,
    /// The MCU answers nothing.
    pub silent: bool,
    /// The patch semaphore answers this, whatever is asked.
    pub sem: Option<u8>,
    /// What the patch finish answers (0 is success).
    pub finish: u8,
    /// The firmware starts but never sets its ready bits.
    pub never_ready: bool,
    /// Channel requests are never granted.
    pub no_grant: bool,
    /// Grants name a token other than the request's.
    pub stale_grant: bool,
    /// The status unified commands answer with.
    pub uni_status: u32,
    /// No transmit-free report is ever sent.
    pub no_txfree: bool,
}
