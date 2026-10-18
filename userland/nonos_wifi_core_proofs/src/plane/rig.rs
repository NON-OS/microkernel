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

//! A fresh soft-MAC station over the simulated WPA2 air, and a count of the
//! non-EAPOL data frames it put out, for the plain-air data-plane proofs.

use crate::soft::ap::SimAp;
use crate::soft::frames::RSNE_PSK;
use crate::soft::join::{PASS, SSID};
use crate::soft::rig::{rig, Station};

/// A fresh station in range of a WPA2 AP that sends data once keyed.
pub fn fresh() -> Station {
    let mut ap = SimAp::new(SSID, PASS, 6, &RSNE_PSK);
    ap.chatter = true;
    rig(ap)
}

/// Non-EAPOL data frames the station put on the air.
pub fn data_frames(st: &Station) -> usize {
    st.radio()
        .unwrap()
        .sent
        .iter()
        .filter(|f| (f[0] >> 2) & 3 == 2)
        .filter(|f| {
            let hdr = if f[0] & 0x80 != 0 { 26 } else { 24 };
            f.get(hdr..hdr + 8) != Some(&[0xaa, 0xaa, 3, 0, 0, 0, 0x88, 0x8e])
        })
        .count()
}
