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

//! IDENTIFY: the unit's INQUIRY vendor and product as its model, its VPD
//! serial, and the medium, UFS. A serial longer than the wire's 20 bytes
//! keeps its last 20, where serials differ.

use crate::protocol::{encode_identify, Request, E_NODEV, MEDIUM_UFS};
use crate::server::reply;
use crate::ufs::disk::Unit;

pub fn serve(u: &Unit, req: &Request, tx: &mut [u8]) {
    let mut model = [b' '; 25];
    let vendor = trim(&u.name[..8]);
    model[..vendor.len()].copy_from_slice(vendor);
    let at = vendor.len() + 1;
    model[at..at + 16].copy_from_slice(&u.name[8..24]);
    let serial = trim(&u.serial[..u.serial_len]);
    let serial = &serial[serial.len().saturating_sub(20)..];
    match encode_identify(&model[..at + 16], serial, MEDIUM_UFS) {
        Some(body) => reply::body(tx, req, &body),
        None => reply::status(tx, req, E_NODEV),
    }
}

/// Without the spaces SCSI pads its fields with, at either end.
fn trim(b: &[u8]) -> &[u8] {
    let start = b.iter().position(|&c| c != b' ').unwrap_or(b.len());
    let end = b.iter().rposition(|&c| c != b' ').map_or(start, |i| i + 1);
    &b[start..end]
}
