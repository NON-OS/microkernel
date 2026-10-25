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

//! `nsh-enrol/1`'s fields before the signature, little-endian, in the
//! order `docs/design/nsh-v1-attest.md` gives.

use nsh_wire::BOOT_APPS_VERSION;

pub struct Fields<'a> {
    pub fleet: [u8; 16],
    pub host: &'a [u8],
    pub ak: [u8; 64],
    pub host_keys: [u8; 64],
    pub pcrs: [[u8; 32]; 4],
    pub boot_apps: &'a [[u8; 32]],
    pub not_after_ms: u64,
    pub authority_id: [u8; 32],
}

/// The caller has checked every value: a host name of at most 253 bytes
/// and at most eight boot applications fit their one-byte counts.
pub fn body(f: &Fields<'_>) -> Vec<u8> {
    let mut out = b"NSHE".to_vec();
    out.extend(1u16.to_le_bytes());
    out.extend(0u16.to_le_bytes());
    out.extend(f.fleet);
    out.push(f.host.len() as u8);
    out.extend(f.host);
    out.extend(f.ak);
    out.extend(f.host_keys);
    f.pcrs.iter().for_each(|p| out.extend(p));
    out.extend([BOOT_APPS_VERSION, f.boot_apps.len() as u8]);
    f.boot_apps.iter().for_each(|a| out.extend(a));
    out.extend(f.not_after_ms.to_le_bytes());
    out.extend(f.authority_id);
    out
}
