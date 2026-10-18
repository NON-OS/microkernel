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

//! The UCSI _DSM (recalled from Intel's UCSI BIOS implementation guide,
//! unverified): function 0 lists the functions, 1 tells the PPM the OPM
//! wrote CONTROL, 2 has firmware copy the PPM's state into the mailbox.

use crate::boot::evaluator;
use crate::host::CapsuleHost;
use alloc::vec;
use nonos_aml::load::Aml;
use nonos_aml::namespace::Id;
use nonos_aml::object::Object;
use nonos_aml::platform::child;

/// 6F8398C2-7CA4-11E4-AD36-631042B5008F, in ToUUID's byte order.
const UUID: [u8; 16] = [
    0xC2, 0x98, 0x83, 0x6F, 0xA4, 0x7C, 0xE4, 0x11, 0xAD, 0x36, 0x63, 0x10, 0x42, 0xB5, 0x00, 0x8F,
];
pub const WRITE: u64 = 1;
pub const READ: u64 = 2;

pub fn call(aml: &mut Aml, host: &mut CapsuleHost, dev: Id, rev: u64, f: u64) -> Option<Object> {
    let args = vec![
        Object::buffer(UUID.to_vec()),
        Object::Integer(rev),
        Object::Integer(f),
        Object::package(vec![]),
    ];
    child(aml, &mut evaluator(host), dev, *b"_DSM", args)?.ok()
}

/// The revision whose function 0 lists both functions this needs, of 1
/// and then 0; None when neither does.
pub fn query(aml: &mut Aml, host: &mut CapsuleHost, dev: Id) -> Option<u64> {
    [1, 0].into_iter().find(|rev| {
        let bits = match call(aml, host, dev, *rev, 0) {
            Some(Object::Buffer(b)) => b.borrow().first().copied().unwrap_or(0),
            Some(Object::Integer(v)) => v as u8,
            _ => 0,
        };
        let need = 1 | 1 << WRITE | 1 << READ;
        bits & need == need
    })
}
