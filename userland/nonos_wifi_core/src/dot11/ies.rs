// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The elements of a beacon or probe response a join needs beyond what a scan
//! shows: the RSN and RSN Extension elements whole (message 3 is compared
//! with them byte for byte), and the rates. The association request offers
//! the access point's own legacy rates back (the radios run every 802.11a/b/g
//! rate), so an AP whose basic rates are OFDM-only, as on a g-only or 5 GHz
//! network, never refuses the station for lacking one. BSS membership
//! selectors ride in the rates elements with the basic bit set and are not
//! rates; they are read here to learn that the AP insists on 802.11n or on
//! SAE hash-to-element.

use super::header::{fc_subtype, fc_type, MAC_HEADER_LEN, SUBTYPE_BEACON, SUBTYPE_PROBE_RESP, TYPE_MGMT};
use super::parse::FIXED_FIELDS_LEN;

/// Element ids.
pub const EID_SUPPORTED_RATES: u8 = 1;
pub const EID_EXT_SUPPORTED_RATES: u8 = 50;
pub const EID_RSN: u8 = 48;
pub const EID_RSNX: u8 = 244;
/// The most rates the Supported Rates element carries; the rest go in
/// Extended Supported Rates.
pub const SUPPORTED_RATES_MAX: usize = 8;
/// The fastest legacy rate, 54 Mb/s, in 500 kb/s units; a larger value in a
/// rates element is a BSS membership selector.
const RATE_MAX: u8 = 108;
/// BSS membership selectors (Table 9-80), without the basic bit.
const SELECTOR_HT_PHY: u8 = 127;
const SELECTOR_SAE_H2E_ONLY: u8 = 123;
/// The 802.11b/g set offered when an AP lists no rates of its own.
const DEFAULT_RATES: [u8; 12] = [0x82, 0x84, 0x8b, 0x96, 0x0c, 0x12, 0x18, 0x24, 0x30, 0x48, 0x60, 0x6c];

/// The tagged-parameter area of a beacon or probe response.
pub fn beacon_tags(frame: &[u8]) -> Option<&[u8]> {
    let start = MAC_HEADER_LEN + FIXED_FIELDS_LEN;
    if frame.len() < start {
        return None;
    }
    let fc = u16::from_le_bytes([frame[0], frame[1]]);
    let sub = fc_subtype(fc);
    if fc_type(fc) != TYPE_MGMT || (sub != SUBTYPE_BEACON && sub != SUBTYPE_PROBE_RESP) {
        return None;
    }
    Some(&frame[start..])
}

/// An element by id, whole (id and length included). A walk that meets a
/// length running past the area stops there.
pub fn find_element(tags: &[u8], id: u8) -> Option<&[u8]> {
    let mut off = 0usize;
    while off + 2 <= tags.len() {
        let end = (off + 2).checked_add(tags[off + 1] as usize)?;
        if end > tags.len() {
            return None;
        }
        if tags[off] == id {
            return Some(&tags[off..end]);
        }
        off = end;
    }
    None
}

/// The rates to offer an access point, and what its selectors require.
#[derive(Clone, Copy)]
pub struct Rates {
    pub rates: [u8; 16],
    pub len: usize,
    /// The AP admits only 802.11n (HT) stations.
    pub requires_ht: bool,
    /// The AP admits only SAE hash-to-element.
    pub requires_h2e: bool,
}

impl Rates {
    /// The Supported Rates part (at most eight).
    pub fn supported(&self) -> &[u8] {
        &self.rates[..self.len.min(SUPPORTED_RATES_MAX)]
    }

    /// The Extended Supported Rates part (possibly empty).
    pub fn extended(&self) -> &[u8] {
        &self.rates[self.len.min(SUPPORTED_RATES_MAX)..self.len]
    }
}

/// The access point's legacy rates (basic bits kept) from its rates
/// elements, without membership selectors; the 802.11b/g set if it lists none.
pub fn ap_rates(tags: &[u8]) -> Rates {
    let mut r = Rates { rates: [0u8; 16], len: 0, requires_ht: false, requires_h2e: false };
    for id in [EID_SUPPORTED_RATES, EID_EXT_SUPPORTED_RATES] {
        let Some(elem) = find_element(tags, id) else {
            continue;
        };
        for &v in &elem[2..] {
            match v & 0x7F {
                SELECTOR_HT_PHY if v & 0x80 != 0 => r.requires_ht = true,
                SELECTOR_SAE_H2E_ONLY if v & 0x80 != 0 => r.requires_h2e = true,
                rate @ 2..=RATE_MAX if rate != 0 && r.len < r.rates.len() => {
                    r.rates[r.len] = v;
                    r.len += 1;
                }
                _ => {}
            }
        }
    }
    if r.len == 0 {
        r.rates[..DEFAULT_RATES.len()].copy_from_slice(&DEFAULT_RATES);
        r.len = DEFAULT_RATES.len();
    }
    r
}
