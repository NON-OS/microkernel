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

use crate::kernel_core::surface_registry::types::{
    RegistryError, SurfaceDescriptor, SurfaceHandle,
};

/*
 * An attach `receiver_pid` already holds for `handle`: fill `out_desc` with
 * its window and return its base VA. None when there is no such record.
 */
pub(super) fn existing(
    receiver_pid: u32,
    handle: SurfaceHandle,
    out_desc: &mut SurfaceDescriptor,
) -> Option<Result<u64, RegistryError>> {
    let (base_va, byte_len) = super::super::attach_map::lookup(receiver_pid, handle)?;
    Some(super::descriptor::descriptor(handle).map(|desc| {
        *out_desc = desc;
        out_desc.base_va = base_va;
        out_desc.byte_len = byte_len;
        base_va
    }))
}
