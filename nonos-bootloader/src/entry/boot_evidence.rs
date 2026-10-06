// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! What the kernel checks this loader with, gathered before boot services
//! end: the firmware's TCG log, the loader's own v4 trailer and the signed
//! boot-root record, the last two from the ESP the loader came from. Each is
//! a zero region when it is absent, and the kernel then says what it lacked.

use alloc::vec::Vec;

use nonos_boot::handoff::types::{
    Module, MODULE_KIND_BOOT_ROOT_RECORD, MODULE_KIND_BOOT_TRAILER, MODULE_KIND_TCG_LOG,
};
use nonos_boot::loader::file::load_file_from_esp;
use nonos_boot::security::tcg_log::event_log;
use uefi::prelude::*;
use uefi::CStr16;

/// The log, the trailer and the record as read, each `None` when absent.
pub struct BootEvidence {
    pub log: Option<&'static [u8]>,
    pub trailer: Option<&'static [u8]>,
    pub record: Option<&'static [u8]>,
}

pub fn boot_evidence(st: &SystemTable<Boot>) -> BootEvidence {
    BootEvidence {
        log: event_log(st.boot_services()),
        trailer: file(st, uefi::cstr16!("\\EFI\\nonos\\bootloader.trailer")),
        record: file(st, uefi::cstr16!("\\EFI\\nonos\\boot_root.approval")),
    }
}

impl BootEvidence {
    /// The regions the kernel is handed, a zero one for each absent.
    pub fn modules(&self) -> [Module; 3] {
        [
            region(self.log, MODULE_KIND_TCG_LOG),
            region(self.trailer, MODULE_KIND_BOOT_TRAILER),
            region(self.record, MODULE_KIND_BOOT_ROOT_RECORD),
        ]
    }
}

/* Leaked into loader memory, which the kernel never reclaims. */
fn file(st: &SystemTable<Boot>, path: &CStr16) -> Option<&'static [u8]> {
    load_file_from_esp(st, path).ok().map(|bytes| &*Vec::leak(bytes))
}

fn region(bytes: Option<&[u8]>, kind: u32) -> Module {
    bytes.map_or(Module::default(), |b| Module {
        base: b.as_ptr() as u64,
        size: b.len() as u64,
        kind,
        reserved: 0,
    })
}
