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

//! Beacon protection (IEEE Std 802.11-2020, 12.5.4.5 and 12.6.x): an AP
//! that runs it ends each Beacon with an MME under the BIGTK. The MIC is BIP's,
//! computed with the Timestamp field taken as zero, since the AP's hardware
//! fills the Timestamp in as the frame goes out.

use alloc::vec::Vec;

use super::bip::{bip_protect, bip_verify};
use super::mmie::Mmie;
use crate::dot11::header::MAC_HEADER_LEN;

/// The Beacon body's Timestamp field, first in the body.
const TIMESTAMP: core::ops::Range<usize> = MAC_HEADER_LEN..MAC_HEADER_LEN + 8;

/// The beacon's MME, if its MIC verifies under `bigtk`.
pub fn bip_verify_beacon(frame: &[u8], bigtk: &[u8; 16]) -> Option<Mmie> {
    let mut f = frame.to_vec();
    f.get_mut(TIMESTAMP)?.fill(0);
    bip_verify(&f, bigtk)
}

/// `frame`, a Beacon, ended by an MME under `bigtk`, its Timestamp kept.
pub fn bip_protect_beacon(
    frame: &[u8],
    key_id: u16,
    bipn: u64,
    bigtk: &[u8; 16],
) -> Option<Vec<u8>> {
    let ts: [u8; 8] = frame.get(TIMESTAMP)?.try_into().ok()?;
    let mut f = frame.to_vec();
    f[TIMESTAMP].fill(0);
    let mut out = bip_protect(&f, key_id, bipn, bigtk)?;
    out[TIMESTAMP].copy_from_slice(&ts);
    Some(out)
}
