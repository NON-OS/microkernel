/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The rate each frame goes at. Linux v6.12 sends the MT7921 family's
//! fixed-rate frames at the lowest of the BSS's basic rates
//! (`mt76_connac2_mac_tx_rate_val`, `ffs(basic_rates) - 1`), or at the
//! band's lowest when none is known (`mt76_calculate_default_rate`), and
//! mac80211 takes the basic rates from the access point's beacon before it
//! authenticates. So here: a frame to the access point goes at the lowest
//! basic rate the access point named (one with 802.11b off is never sent
//! CCK), any other frame at the band's lowest. Data to the access point
//! goes at its fastest basic rate, where Linux leaves the choice to the
//! firmware's rate control; its EAPOL frames go at the lowest, since a
//! handshake frame lost costs the join.

use nonos_wifi_core::dot11::header::{fc_type, header_len};

use super::rate::{lowest, value};

/// The LLC/SNAP header of an EAPOL frame (ethertype 0x888e).
const EAPOL_LLC: [u8; 8] = [0xaa, 0xaa, 3, 0, 0, 0, 0x88, 0x8e];

/// Whether `mpdu` is an EAPOL frame sent in the clear.
fn eapol(mpdu: &[u8], fc: u16) -> bool {
    let at = header_len(fc);
    fc & 0x4000 == 0 && mpdu.get(at..at + 8) == Some(&EAPOL_LLC[..])
}

/// The TXD rate value for `mpdu`. `to_ap`: it is addressed to the access
/// point, whose basic rates (lowest, fastest) are `bss` when known.
pub fn pick(mpdu: &[u8], to_ap: bool, bss: Option<(u8, u8)>, ghz5: bool) -> u16 {
    let fc = u16::from_le_bytes([mpdu[0], mpdu[1]]);
    let band = (lowest(ghz5), lowest(ghz5));
    let (low, fast) = bss.filter(|_| to_ap).unwrap_or(band);
    let data = fc_type(fc) == 2 && !eapol(mpdu, fc);
    value(if data { fast } else { low }, ghz5)
}
