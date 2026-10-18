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

//! The access point's legacy rates (basic bits kept) from its rates
//! elements, without membership selectors; the 802.11b/g set if it lists
//! none.

use super::*;

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
