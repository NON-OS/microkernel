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

//! The end of aarch64 bring-up, wherever the kernel runs by then.

use core::sync::atomic::{AtomicBool, Ordering};

use super::info::BootInfo;
use crate::arch::aarch64::security::{epd1, spectre};
use crate::sys::serial;

/// Whether the device tree was read, for the line `finish` prints: a value,
/// so it means the same wherever the kernel runs.
static PARSED: AtomicBool = AtomicBool::new(false);

pub(super) fn set_parsed(parsed: bool) {
    PARSED.store(parsed, Ordering::Relaxed);
}

pub(super) fn finish(info: &BootInfo) -> ! {
    // Before any other CPU starts, so each exposed one finds the trampoline
    // built when it decides, and before any user root exists.
    epd1::activate(spectre::boot_exposed());
    super::init::bring_up(info);
    epd1::report();
    // After the secondaries are up: until then each waits on a plain WFI.
    crate::arch::aarch64::idle_states::prepare();

    #[cfg(feature = "nonos-trap-proof-kernel-abort")]
    super::trap_proof::kernel_data_abort();

    if PARSED.load(Ordering::Relaxed) {
        serial::println(b"[NONOS] aarch64 boot init done");
    } else {
        // The defaults describe QEMU virt. Say so: on any other board they are
        // wrong, and a silent boot into the wrong MMIO is worse than a loud one.
        serial::println(b"[NONOS] no usable device tree, assuming QEMU virt");
    }
    // Where the kernel now stays, so the shared readers' reference holds.
    if let Err(e) = super::loader::install() {
        super::refuse(b"loader handoff", e.as_str().as_bytes());
    }
    super::core_entry::enter(info)
}
