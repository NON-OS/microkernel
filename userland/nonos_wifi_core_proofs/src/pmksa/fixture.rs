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

//! A cache, its tags and entries, for the cache proofs.

use nonos_wifi_core::pmksa::{Pmksa, PmksaCache, Tag};
use nonos_wifi_core::vault::Vault;

pub const SSID: &[u8] = b"home";
pub const PASS: &[u8] = b"correct horse";

pub const STA: [u8; 6] = [2, 0, 0, 0, 0, 0x5a];

/// A cache keyed by a fixed tag key, as a start's randomness keys one.
pub fn cache() -> PmksaCache {
    PmksaCache::new(Vault::new(&[0x3c; 32]))
}

pub fn tag(c: &PmksaCache, ssid: &[u8], pass: &[u8]) -> Tag {
    c.tag(ssid, pass).expect("a tag")
}

/// A second owned PMKSA equal to `p`: the type is neither Copy nor Clone.
pub fn twin(p: &Pmksa) -> Pmksa {
    Pmksa { bssid: p.bssid, pmk: p.pmk, pmkid: p.pmkid }
}

pub fn sa(n: u8) -> Pmksa {
    Pmksa { bssid: [2, 0, 0, 0, 0, n], pmk: [n; 32], pmkid: [n ^ 0x55; 16] }
}
