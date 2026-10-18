// NONOS Operating System (AGPL-3.0-or-later)

//! EAPOL frames to and from the access point.

use super::classify::{ETHERTYPE_EAPOL, ETH_HEADER_LEN};
use alloc::vec::Vec;
use nonos_wifi_core::dot11::data::build_data;
use nonos_wifi_core::dot11::header::MAC_HEADER_LEN;
use nonos_wifi_core::frame::LLC_SNAP;

/// Where the EAPOL payload begins in a data frame, or `None` if it carries none.
/// Rather than compute the header length (which varies with QoS, HT-control and
/// four-address frames, and which an access point can combine in ways a fixed
/// offset gets wrong), this searches for the LLC/SNAP shim followed by the EAPOL
/// ethertype anywhere past the MAC header. The handshake frames are unencrypted,
/// so the signature appears in the clear.
pub(super) fn eapol_payload_start(frame: &[u8]) -> Option<usize> {
    if frame.len() < MAC_HEADER_LEN + 8 {
        return None;
    }
    for pos in MAC_HEADER_LEN..=frame.len() - 8 {
        if frame[pos..pos + 6] == LLC_SNAP
            && u16::from_be_bytes([frame[pos + 6], frame[pos + 7]]) == ETHERTYPE_EAPOL
        {
            return Some(pos + 8);
        }
    }
    None
}

/// Wrap an EAPOL payload into an unencrypted 802.11 data frame to the AP. The
/// keys are not installed yet, so the handshake frames go out in the clear, which
/// is exactly what the standard expects. Builds the ethernet frame the shared
/// encapsulator wants, then hands it to `build_data`.
pub fn wrap_eapol(eapol: &[u8], our_mac: [u8; 6], bssid: [u8; 6], seq: u16) -> Option<Vec<u8>> {
    let mut eth = Vec::with_capacity(ETH_HEADER_LEN + eapol.len());
    eth.extend_from_slice(&bssid); // destination: the AP
    eth.extend_from_slice(&our_mac); // source: the station
    eth.extend_from_slice(&ETHERTYPE_EAPOL.to_be_bytes());
    eth.extend_from_slice(eapol);
    build_data(&eth, our_mac, bssid, seq)
}

/// Whether a data frame carrying EAPOL is the AP's to this station: FromDS,
/// transmitter the BSSID, receiver this station, and not protected.
pub fn eapol_from_bss(frame: &[u8], our_mac: &[u8; 6], bssid: &[u8; 6]) -> bool {
    frame.len() >= MAC_HEADER_LEN
        && frame[1] & 0x03 == 0x02
        && frame[1] & 0x40 == 0
        && frame[4..10] == our_mac[..]
        && frame[10..16] == bssid[..]
}
