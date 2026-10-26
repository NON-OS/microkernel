// NONOS Operating System (AGPL-3.0-or-later)

//! The radio a join drives, and what it counted.

/// The radio the association driver transmits on and receives from. The driver
/// implements this over its transmit and receive rings; the proofs implement it
/// over a scripted exchange.
pub trait Radio {
    /// Transmit one complete 802.11 frame (management or data). Returns false if
    /// it could not be queued.
    fn send(&mut self, mpdu: &[u8]) -> bool;
    /// Copy the next received 802.11 frame into `out`, returning its length, or
    /// `None` if nothing is waiting right now.
    fn recv(&mut self, out: &mut [u8]) -> Option<usize>;
}

/// The running progress counters for one join.
#[derive(Default)]
pub(super) struct Counters {
    pub(super) sent: u32,
    pub(super) recv: u32,
    pub(super) data: u32,
    pub(super) eapol: u32,
    /// The first non-EAPOL data frame's frame-control in the low 16 bits and its
    /// length in the high 16, for diagnosis: the protected bit and subtype in the
    /// frame-control, and whether the length is handshake-sized, say what the frame
    /// the card delivered actually is.
    pub(super) probe: u32,
    /// Deauthenticate/disassociate management frames received.
    pub(super) deauth: u32,
    /// Data frames addressed (receiver address) to our own MAC.
    pub(super) to_us: u32,
    /// The AID the access point granted, and its DTIM period.
    pub(super) aid: u16,
    pub(super) dtim: u8,
}
