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

//! Calling the PCI `_DSM`: function 0 first, for the mask of what firmware
//! implements at a revision, then only a documented function the mask has.

use crate::load::Aml;
use crate::namespace::Id;
use crate::object::Object;
use crate::platform::{child, offered_mask, offers, Eval};
use alloc::vec;

/// E5C937D0-3553-4D7A-9117-EA4D19C3434D, in ToUUID's byte order.
pub const PCI_DSM_UUID: [u8; 16] = [
    0xD0, 0x37, 0xC9, 0xE5, 0x53, 0x35, 0x7A, 0x4D, 0x91, 0x17, 0xEA, 0x4D, 0x19, 0xC3, 0x43, 0x4D,
];
/// Newest first: firmware may answer only a revision it knows.
const REVISIONS: [u64; 2] = [3, 2];
/// The only functions read: 5 ignore boot configuration, 7 device label,
/// 9 device readiness durations.
pub const DOCUMENTED: [u64; 3] = [5, 7, 9];

fn dsm(aml: &mut Aml, eval: &mut Eval<'_>, dev: Id, rev: u64, f: u64) -> Option<Object> {
    let uuid = Object::buffer(PCI_DSM_UUID.to_vec());
    let args = vec![uuid, Object::Integer(rev), Object::Integer(f), Object::package(vec![])];
    child(aml, eval, dev, *b"_DSM", args)?.ok()
}

/// The first revision whose function 0 says anything is there, and its
/// mask (bit n for function n).
pub fn offered(aml: &mut Aml, eval: &mut Eval<'_>, dev: Id) -> Option<(u64, u64)> {
    REVISIONS.iter().find_map(|&rev| {
        let mask = offered_mask(dsm(aml, eval, dev, rev, 0).as_ref());
        (mask != 0).then_some((rev, mask))
    })
}

/// Function `f`'s answer, when it is documented and offered; never asked
/// otherwise.
pub fn ask(aml: &mut Aml, eval: &mut Eval<'_>, dev: Id, f: u64) -> Option<Object> {
    if !DOCUMENTED.contains(&f) {
        return None;
    }
    let (rev, mask) = offered(aml, eval, dev)?;
    offers(mask, f).then(|| dsm(aml, eval, dev, rev, f)).flatten()
}
