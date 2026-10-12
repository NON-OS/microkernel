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

//! Whether a scope around a processor is a processor container (ACPI
//! 6.5, 8.4.2): a device whose _HID is ACPI0010, named by its _UID.

use crate::load::Aml;
use crate::namespace::{Id, Kind};
use crate::platform::{child, hardware_id, integer, Eval};
use alloc::vec::Vec;

pub const CONTAINER_HID: &[u8] = b"ACPI0010";

/// Some(None) for a scope that is no container, Some(Some(uid)) for one,
/// and None for a container with no _UID.
pub(super) fn container_uid(aml: &mut Aml, eval: &mut Eval<'_>, id: Id) -> Option<Option<u64>> {
    if aml.ns.get(id).map(|e| e.kind) != Some(Kind::Device) {
        return Some(None);
    }
    let hid = child(aml, eval, id, *b"_HID", Vec::new()).and_then(|r| r.ok());
    if hid.as_ref().and_then(hardware_id).is_none_or(|h| h.bytes() != CONTAINER_HID) {
        return Some(None);
    }
    integer(&child(aml, eval, id, *b"_UID", Vec::new())?.ok()?).map(Some)
}
