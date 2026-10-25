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

//! The load-by-name request as the installer decodes it: header, caps, name, argv.

use alloc::vec::Vec;

// None named: the installer asks for the capsule's own manifest set, and the
// kernel refuses a request beyond the manifest, so the Terminal never picks
// what a capsule it launches holds.
const REQUESTED_CAPS: u64 = 0;

// Installer wire: seq(4) | op(2) | pad(2) | body. Mirrors the installer's
// decode_request, which reads the op at byte 4 and the body after an 8 byte
// header. The reply is seq(4) | status(4) | pid(4), and the installer omits
// the pid on failure, so only a success reply is 12 bytes long.
const OP_LOAD_BY_NAME: u16 = 4;
const SEQ: u32 = 1;

pub(super) fn pack(name: &[u8], args: &[u8]) -> Vec<u8> {
    let mut p = Vec::with_capacity(8 + 9 + name.len() + args.len());
    p.extend_from_slice(&SEQ.to_le_bytes());
    p.extend_from_slice(&OP_LOAD_BY_NAME.to_le_bytes());
    p.extend_from_slice(&[0u8, 0u8]);
    p.extend_from_slice(&REQUESTED_CAPS.to_le_bytes());
    p.push(name.len() as u8);
    p.extend_from_slice(name);
    p.extend_from_slice(args);
    p
}
