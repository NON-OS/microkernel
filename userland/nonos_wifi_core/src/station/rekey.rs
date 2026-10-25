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

//! A pairwise rekey taken: the station's own copy of the key, where it runs
//! the cipher in software, replaced in place, with its packet numbers and
//! replay counters started afresh (IEEE Std 802.11-2020, 12.5.3.4.4), the
//! protected management frames' among them; and the seed the next rekey's
//! SNonce is drawn from.

use super::{Ccmp, LinkStation};

impl LinkStation {
    /// Take `tk` as the pairwise key from here on. A chip that runs CCMP in
    /// hardware takes it from the driver (`Rx::Handshake::pairwise_key`).
    pub(super) fn set_pairwise_key(&mut self, tk: &[u8]) {
        match &mut self.ccmp {
            Ccmp::Software { tk: old } | Ccmp::SoftwareTxHwRx { tk: old } => {
                if let Ok(k) = tk.try_into() {
                    *old = k;
                }
            }
            Ccmp::Gcmp256 { tk: old } => {
                if let Ok(k) = tk.try_into() {
                    *old = k;
                }
            }
            Ccmp::Hardware => {}
        }
        self.pn = 0;
        self.replay.reset_unicast();
        self.pmf.mgmt_pn = 0;
    }

    /// A fresh seed for the next rekey's SNonce, drawn by the driver once a
    /// rekey has finished (`Rx::Handshake::pairwise_key`).
    pub fn reseed_rekey(&mut self, seed: [u8; 32]) {
        if let Some(s) = self.supplicant.as_mut() {
            s.reseed_rekey(seed);
        }
    }
}
