// NONOS Operating System (AGPL-3.0-or-later)

//! What the radio has seen go wrong, for the status op's diagnostic words.
//! Counters saturate; none of them carries an address or a frame's bytes.

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Counts {
    /// Firmware commands (or mailbox messages) that failed after bring-up.
    pub cmd_fail: u16,
    /// Frames the rings could not take.
    pub tx_full: u16,
    /// Receive buffers dropped: split, short, or not a frame.
    pub rx_drop: u16,
    /// Frames dropped for a bad CRC or ICV.
    pub rx_bad_crc: u16,
    /// Channel switches the firmware did not acknowledge.
    pub tune_fail: u16,
}

pub fn bump(c: &mut u16) {
    *c = c.saturating_add(1);
}
