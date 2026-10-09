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

//! The modules the loader left for this check, as byte slices.

use crate::boot::handoff::types::{
    MODULE_KIND_BOOT_ROOT_RECORD, MODULE_KIND_BOOT_TRAILER, MODULE_KIND_LOADER_IMAGE,
    MODULE_KIND_TCG_LOG,
};
use crate::security::boot::modules::module_bytes;

/// The longest TCG log, record or trailer this check reads.
const MAX_MODULE: u64 = 4 * 1024 * 1024;
/// The longest loader image it reads. The loader as built is near 15 MiB, so
/// the 4 MiB bound above refused every one, and a boot whose TPM held no
/// rollback index was left with no evidence and stopped.
const MAX_LOADER_IMAGE: u64 = 64 * 1024 * 1024;

pub(super) struct Evidence {
    pub log: Option<&'static [u8]>,
    pub trailer: Option<&'static [u8]>,
    pub record: Option<&'static [u8]>,
    pub loader: Option<&'static [u8]>,
}

pub(super) fn gather() -> Evidence {
    Evidence {
        log: bytes(MODULE_KIND_TCG_LOG),
        trailer: bytes(MODULE_KIND_BOOT_TRAILER),
        record: bytes(MODULE_KIND_BOOT_ROOT_RECORD),
        loader: module_bytes(MODULE_KIND_LOADER_IMAGE, MAX_LOADER_IMAGE),
    }
}

fn bytes(kind: u32) -> Option<&'static [u8]> {
    module_bytes(kind, MAX_MODULE)
}
