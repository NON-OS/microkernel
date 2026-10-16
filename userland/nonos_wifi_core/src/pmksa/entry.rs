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

//! One PMK security association (IEEE Std 802.11-2020, 12.6.10.3): the PMK
//! an SAE exchange with one AP produced, and the PMKID that names it. With
//! one cached, a later join to the same AP may skip SAE: Open System
//! authentication, then a (Re)Association Request whose RSNE carries the
//! PMKID, and the four-way handshake under the cached PMK.

use crate::vault::{assert_not_copy, wipe};

/// The PMK and the PMKID SAE gave for one AP. Never Copy, Clone or Debug:
/// each value is wiped when dropped, and a second one is made only on
/// purpose (`again`), so no copy of the PMK is left behind unwiped.
#[derive(PartialEq, Eq)]
pub struct Pmksa {
    pub bssid: [u8; 6],
    pub pmk: [u8; 32],
    pub pmkid: [u8; 16],
}

assert_not_copy!(Pmksa);

impl Pmksa {
    /// A second owned value, for a holder that keeps its own and wipes it.
    pub(crate) fn again(&self) -> Self {
        Self { bssid: self.bssid, pmk: self.pmk, pmkid: self.pmkid }
    }
}

impl Drop for Pmksa {
    fn drop(&mut self) {
        wipe(&mut self.pmk);
        wipe(&mut self.pmkid);
    }
}

/// One entry as the cache keeps it.
pub(crate) struct Kept {
    /// The station address it was made from, so a new address joins anew.
    pub(crate) sta: [u8; 6],
    pub(crate) ssid: [u8; 32],
    pub(crate) ssid_len: usize,
    pub(crate) since_ms: u64,
    /// The passphrase's `Tag`.
    pub(crate) tag: [u8; 32],
    pub(crate) sa: Pmksa,
}

impl Drop for Kept {
    fn drop(&mut self) {
        wipe(&mut self.tag);
    }
}
