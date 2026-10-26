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

//! The logger's start and its shutdown wipe.

use super::state::LOGGER;

/// Start the logger's backends, once, from boot as soon as the heap is up
/// (every log macro formats into the heap). The RAM log needs no start.
/// There is no screen backend: the kernel's one x86 entry is the NONOS
/// loader's UEFI handoff, whose console is the GOP framebuffer, so no VGA
/// text console exists to mirror the log onto. The serial echo is the
/// attack suite's and the serial-debug images' (backend/serial.rs).
pub fn init() {
    #[cfg(any(feature = "nonos-capsule-attack", feature = "capsule-serial-debug"))]
    if let Some(mgr) = LOGGER.lock().as_mut() {
        mgr.add_backend(alloc::boxed::Box::new(crate::log::backend::SerialBackend::new()));
    }
}

/// The shutdown wipe's: overwrite what the logger holds in RAM. It never
/// waits for the lock, since a holder may be the very CPU that is shutting
/// down; a log held at that moment keeps its text until power goes.
pub fn wipe_ram_log() -> bool {
    match LOGGER.try_lock() {
        Some(mut l) => {
            if let Some(mgr) = l.as_mut() {
                mgr.wipe();
            }
            true
        }
        None => false,
    }
}
