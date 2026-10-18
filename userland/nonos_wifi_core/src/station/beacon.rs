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

//! The session AP's beacons under beacon protection. Once the handshake
//! delivered a BIGTK, a beacon from the AP counts only if it ends in an MME
//! under that key's index, its MIC verifies, and its BIPN climbs above the
//! last one taken. Anyone can send a beacon in the AP's name; only the AP
//! holds the BIGTK.

use super::mgmt_rx::MgmtDrop;
use super::LinkStation;
use crate::dot11::header::MAC_HEADER_LEN;
use crate::pmf::{bip_verify_beacon, Mmie};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BeaconRx {
    /// The link runs no beacon protection: no BIGTK was delivered.
    Unprotected,
    /// The AP's own beacon, verified and fresh.
    Verified,
    /// Not to be used, for this reason.
    Refused(MgmtDrop),
}

impl LinkStation {
    /// Check one Beacon frame (FCS removed) from the air.
    pub fn receive_beacon(&mut self, f: &[u8]) -> BeaconRx {
        let Some((key, id)) = self.pmf.bigtk.filter(|_| self.associated) else {
            return BeaconRx::Unprotected;
        };
        match self.beacon_checked(f, &key, id) {
            Ok(()) => BeaconRx::Verified,
            Err(why) => BeaconRx::Refused(why),
        }
    }

    fn beacon_checked(&mut self, f: &[u8], key: &[u8; 16], id: u16) -> Result<(), MgmtDrop> {
        if f.len() < MAC_HEADER_LEN + 12 || f[0] != 0x80 {
            return Err(MgmtDrop::Malformed);
        }
        if f[10..16] != self.bssid || f[16..22] != self.bssid {
            return Err(MgmtDrop::NotOurs);
        }
        let mmie = Mmie::from_tail(&f[MAC_HEADER_LEN..]).ok_or(MgmtDrop::Unprotected)?;
        if mmie.key_id != id || bip_verify_beacon(f, key).is_none() {
            return Err(MgmtDrop::Undecryptable);
        }
        if mmie.ipn <= self.pmf.bipn_floor {
            return Err(MgmtDrop::Replay);
        }
        self.pmf.bipn_floor = mmie.ipn;
        Ok(())
    }
}
