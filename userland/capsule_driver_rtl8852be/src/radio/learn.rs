// NONOS Operating System (AGPL-3.0-or-later)

//! The AP's rates from its beacons and probe responses, while joining:
//! mac80211 sets `bss_conf.basic_rates` from the BSS before it
//! authenticates, and rtw89 sends management frames to the AP at the lowest
//! of them. Only frames whose BSSID is the one being joined count, and only
//! until association, when the association's own rates take over.

use crate::h2c::media::Net;
use crate::sta::chip::Vif;
use crate::tx::rate::learn;

/// Fixed fields of a beacon or probe response after the 24-byte header:
/// timestamp, interval, capability.
const BODY_FIXED: usize = 12;

pub fn from_frame(v: &mut Vif, f: &[u8]) {
    if v.net == Net::Infra || v.bssid == [0; 6] || f.len() < 24 + BODY_FIXED {
        return;
    }
    let fc = u16::from_le_bytes([f[0], f[1]]);
    let beacon_or_resp = fc & 0xfc == 0x80 || fc & 0xfc == 0x50;
    if !beacon_or_resp || f[16..22] != v.bssid {
        return;
    }
    let mut rates = [0u8; 16];
    let n = learn(&f[24 + BODY_FIXED..], &mut rates);
    if n > 0 {
        v.rates = rates;
        v.n_rates = n;
    }
}
