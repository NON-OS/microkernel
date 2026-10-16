// NONOS Operating System (AGPL-3.0-or-later)

//! Which kind of frame came in.

use super::eapol::eapol_payload_start;
use alloc::vec::Vec;
use nonos_wifi_core::dot11::header::{TYPE_DATA, TYPE_MGMT};

/// The EtherType that marks an 802.1X (EAPOL) payload.
pub(super) const ETHERTYPE_EAPOL: u16 = 0x888E;

/// An ethernet header is two MACs and the ethertype.
pub(super) const ETH_HEADER_LEN: usize = 14;

/// What a received frame is, to the join state machine.
#[derive(Debug, PartialEq, Eq)]
pub enum RxKind {
    /// A management frame, fed to the machine whole.
    Mgmt,
    /// A data frame carrying an EAPOL payload at this byte range.
    Eapol(usize, usize),
    /// Anything else: ignored while joining.
    Other,
}

/// The frame control field's type bits.
pub(super) fn fc_type(frame: &[u8]) -> Option<u8> {
    if frame.len() < 2 {
        return None;
    }
    let fc = u16::from_le_bytes([frame[0], frame[1]]);
    Some(((fc >> 2) & 0x3) as u8)
}

/// Classify a received 802.11 frame for the join. A management frame is handed to
/// the machine whole; a data frame is unwrapped and, if it carries EAPOL, its
/// payload range within `eth` is returned so the caller can feed it.
pub fn classify(frame: &[u8], eth: &mut Vec<u8>) -> RxKind {
    match fc_type(frame) {
        Some(TYPE_MGMT) => RxKind::Mgmt,
        Some(TYPE_DATA) => match eapol_payload_start(frame) {
            Some(pos) => {
                eth.clear();
                eth.extend_from_slice(&frame[pos..]);
                RxKind::Eapol(0, eth.len())
            }
            None => RxKind::Other,
        },
        _ => RxKind::Other,
    }
}
