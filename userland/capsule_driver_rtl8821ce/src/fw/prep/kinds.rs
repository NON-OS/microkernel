// NONOS Operating System (AGPL-3.0-or-later)

//! How the download ended, and what it saved.

/// How the download close-out ended, so a serial-less boot can tell the two
/// distinct failures apart from the ready case.
pub enum Finish {
    /// The 8051 booted and reported the firmware loaded and initialised.
    Ready,
    /// The hardware did not confirm both section checksums: the bytes the DDMA
    /// delivered did not match. Carries the control register for the console.
    ChecksumBad(u16),
    /// Checksums were good but the 8051 never reached ready within the poll.
    /// Carries the last control register read, whose bits say how far it got.
    NotReady(u16),
}

/// The registers `begin` overwrote, saved so `finish` can put them back exactly.
#[derive(Clone, Copy, Default)]
pub struct Backup {
    pub(super) pq_map: u8,
    pub(super) cr: u8,
    pub(super) fifopage_info: u16,
    pub(super) rqpn: u32,
    pub(super) bcn_ctrl: u8,
}
