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

//! What an accepted association settles, for a chip that keeps its own
//! copy: AP, AID, capability and rates from the response, the beacon
//! interval advertised. mac80211 hands drivers the same through
//! `bss_info_changed`, before the four-way handshake.

use super::radio::Radio;
use crate::dot11::ies::find_element;
use crate::mlme::MlmeState;

const HDR: usize = 24;
const ELEMENT_RATES: u8 = 1;
const ELEMENT_EXT_RATES: u8 = 50;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Association {
    pub bssid: [u8; 6],
    pub aid: u16,
    pub capability: u16,
    pub beacon_int: u16,
    /// Supported then extended rates, 500 kb/s units, bit 7 set on the
    /// basic ones; `n_rates` of them.
    pub rates: [u8; 16],
    pub n_rates: usize,
}

impl Association {
    pub fn parse(resp: &[u8], beacon: &[u8]) -> Option<Self> {
        let fixed = resp.get(HDR..HDR + 6)?;
        let le = |b: &[u8], i: usize| u16::from_le_bytes([b[i], b[i + 1]]);
        let mut a = Association {
            bssid: resp.get(16..22)?.try_into().ok()?,
            aid: le(fixed, 4) & 0x3fff,
            capability: le(fixed, 0),
            beacon_int: beacon.get(HDR + 8..HDR + 10).map_or(0, |b| le(b, 0)),
            rates: [0; 16],
            n_rates: 0,
        };
        let tags = &resp[HDR + 6..];
        let each = |id| find_element(tags, id).and_then(|e| e.get(2..)).unwrap_or(&[]);
        let all = each(ELEMENT_RATES).iter().chain(each(ELEMENT_EXT_RATES));
        for (slot, &r) in a.rates.iter_mut().zip(all) {
            (*slot, a.n_rates) = (r, a.n_rates + 1);
        }
        Some(a)
    }
}

/// The radio told once the AP has accepted us (association to the
/// handshake), from the response that said so.
pub fn announce<R: Radio>(r: &mut R, step: (MlmeState, MlmeState), resp: &[u8], beacon: &[u8]) {
    if step == (MlmeState::Associating, MlmeState::FourWay) {
        if let Some(a) = Association::parse(resp, beacon) {
            r.associated(Some(&a));
        }
    }
}
