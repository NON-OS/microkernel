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

//! The PPI _DSM: its UUID, the TPM device ids that carry it, and the three
//! read functions used here: 0 (the supported functions, a bit each), 1
//! (the interface version, a string such as "1.3") and 8 (for one
//! operation, what the OS may do: 0 not implemented, 1 firmware only, 2
//! blocked by firmware settings, 3 allowed with a physically present user,
//! 4 allowed without). Functions 2, 6 and 7 submit requests and are never
//! called. All recalled from the TCG Physical Presence Interface
//! specification, not read: unverified.

use crate::load::Aml;
use crate::namespace::Id;
use crate::object::Object;
use crate::platform::{child, Eval};
use alloc::vec;

/// 3DDDFAA6-361B-4EB4-A424-8D10089D1653, in ToUUID's byte order.
pub const PPI_UUID: [u8; 16] = [
    0xA6, 0xFA, 0xDD, 0x3D, 0x1B, 0x36, 0xB4, 0x4E, 0xA4, 0x24, 0x8D, 0x10, 0x08, 0x9D, 0x16, 0x53,
];
/// TPM 2.0 and the TPM 1.2-compatible id.
pub const TPM_IDS: [&[u8]; 2] = [b"MSFT0101", b"PNP0C31"];
const REVISION: u64 = 1;

pub(super) const QUERY: u64 = 0;
pub(super) const VERSION: u64 = 1;
pub(super) const USER_CONFIRMATION: u64 = 8;

pub(super) fn call(
    aml: &mut Aml,
    eval: &mut Eval<'_>,
    dev: Id,
    f: u64,
    arg: Option<u64>,
) -> Option<Object> {
    let args = vec![
        Object::buffer(PPI_UUID.to_vec()),
        Object::Integer(REVISION),
        Object::Integer(f),
        Object::package(arg.map(Object::Integer).into_iter().collect()),
    ];
    child(aml, eval, dev, *b"_DSM", args)?.ok()
}
