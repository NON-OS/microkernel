// NONOS Operating System (AGPL-3.0-or-later)

//! Sending a join's frames.

use super::radio::{Counters, Radio};
use alloc::vec::Vec;
use nonos_wifi_core::dot11::header::{seq_control, MAC_HEADER_LEN};
use nonos_wifi_core::mlme::Mlme;

/// The Retry bit of the frame control's second byte.
const FC_RETRY: u8 = 0x08;

/// The Authentication frame's first frame-control byte (management, subtype 11).
pub(super) const FC0_AUTH: u8 = 0xB0;

/// The beacon selects the BSS and produces the authentication request, which
/// goes out at once and is the first frame to resend; none leaves nothing.
pub(super) fn open<R: Radio>(
    radio: &mut R,
    mlme: &mut Mlme,
    beacon: &[u8],
    seq: &mut u16,
    c: &mut Counters,
) -> Vec<u8> {
    let mut pending: Vec<u8> = Vec::new();
    let out = mlme.on_mgmt(beacon);
    if let Some(mut tx) = out.tx {
        transmit(radio, &mut tx, seq, c);
        pending = tx;
    }
    pending
}

/*
 * Send `mpdu` as a new frame: the next number of the join's sequence counter
 * and the Retry bit clear. A resend is a new frame too, as mac80211 sends one.
 * The Retry bit belongs to the MAC's own retransmissions of one frame, which
 * the hardware makes with its number; a receiver discards any frame with the
 * Retry bit set whose number matches the last it took from this station
 * (802.11-2020, 10.3.2.14.3). A resend that kept its number and set the bit was
 * discarded by every access point that had taken the first copy, so when the
 * answer to it was lost, or the answer was status 30 while the access point
 * checked an earlier association with protected management frames, no resend
 * ever reached it and the join timed out.
 */
pub(super) fn transmit<R: Radio>(radio: &mut R, mpdu: &mut [u8], seq: &mut u16, c: &mut Counters) {
    if mpdu.len() >= MAC_HEADER_LEN {
        mpdu[1] &= !FC_RETRY;
        mpdu[22..24].copy_from_slice(&seq_control(*seq).to_le_bytes());
        *seq = seq.wrapping_add(1) & 0x0FFF;
    }
    if radio.send(mpdu) {
        c.sent += 1;
    }
}
