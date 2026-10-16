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

//! What management frame protection keeps on a link: whether it was
//! negotiated, the integrity group keys with their packet-number floors, the
//! floor for protected individually addressed management frames (counted
//! apart from data, IEEE Std 802.11-2020 12.5.3.4.4), and an SA Query the
//! station is running.

use crate::pmf::SaQuery;
use crate::wpa::supplicant::Supplicant;

#[derive(Clone, Copy)]
pub(super) struct PmfState {
    pub(super) on: bool,
    pub(super) igtk: Option<([u8; 16], u16)>,
    pub(super) ipn_floor: u64,
    /// The beacon integrity group key, with the BIPN floor its beacons
    /// must climb above.
    pub(super) bigtk: Option<([u8; 16], u16)>,
    pub(super) bipn_floor: u64,
    pub(super) mgmt_pn: u64,
    pub(super) query: Option<SaQuery>,
}

type Slot = Option<([u8; 16], u16)>;

impl PmfState {
    pub(super) const OFF: Self = Self {
        on: false,
        igtk: None,
        ipn_floor: 0,
        bigtk: None,
        bipn_floor: 0,
        mgmt_pn: 0,
        query: None,
    };

    /// Take the IGTK and BIGTK the supplicant holds. A new key (a rekey
    /// changes the index) starts its floor at the packet number it was
    /// delivered with; the same key keeps the floor it has, so a repeated
    /// message cannot lower it.
    pub(super) fn take_igtk(&mut self, sup: &Supplicant) {
        self.on = sup.pmf();
        if let Some(k) = sup.igtk() {
            take((&mut self.igtk, &mut self.ipn_floor), k);
        }
        if let Some(k) = sup.bigtk() {
            take((&mut self.bigtk, &mut self.bipn_floor), k);
        }
    }
}

fn take(at: (&mut Slot, &mut u64), (key, id, pn): (&[u8; 16], u16, [u8; 6])) {
    if *at.0 == Some((*key, id)) {
        return;
    }
    let mut le = [0u8; 8];
    le[..6].copy_from_slice(&pn);
    (*at.0, *at.1) = (Some((*key, id)), u64::from_le_bytes(le));
}
