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

//! Host proofs, group 4 of 8: test modules mounted by path
//! from proofs.rs, each at its own file.

/// The entropy capsule's bytes, health-tested and mixed.
#[path = "entropy_health/mod.rs"]
pub mod entropy_health;
/// A claimed function is reset at claim and at release.
#[path = "function_reset/mod.rs"]
pub mod function_reset;
/// A holder's grant window and quota.
#[path = "grant_window/mod.rs"]
pub mod grant_window;
/// A capsule over the image ceiling is refused at spawn.
#[path = "image_ceiling/mod.rs"]
pub mod image_ceiling;
/// Each input event's source, stamped by the kernel.
#[path = "input_source/mod.rs"]
pub mod input_source;
/// The input ring's take clears what it hands out.
#[path = "input_take/mod.rs"]
pub mod input_take;
/// IPC payloads zeroed before they are freed (C8).
#[path = "ipc_wipe/mod.rs"]
pub mod ipc_wipe;
/// The restart policy init holds a watched service to.
#[path = "lifecycle_state.rs"]
pub mod lifecycle_state;
/// Names a runtime-loaded capsule may declare.
#[path = "name_rule/mod.rs"]
pub mod name_rule;
/// scrub.S assembled into the host, with its own test (I2).
#[cfg(target_arch = "x86_64")]
#[path = "../../../src/security/hardening/memory_sanitization/scrub_asm.rs"]
mod scrub_asm;
/// An exiting process's surfaces refuse attaches.
#[path = "surface_closing/mod.rs"]
pub mod surface_closing;
#[path = "syscall_tests.rs"]
mod syscall_tests;
/// A wall-clock correction's bounds, with the file's own tests (I2).
#[path = "../../../src/syscall/microkernel/time/offset.rs"]
pub mod time_offset;
#[path = "uefi_cache/mod.rs"]
mod uefi_cache;
#[path = "usable_span_frame_tests.rs"]
mod usable_span_frame_tests;
#[path = "usable_span_tests.rs"]
mod usable_span_tests;
/// driver.usb_msc0's search report in words (usb_msc_report_tests).
#[path = "../../../src/hardware/usb_msc_capsule/report_words.rs"]
pub mod usb_msc_report_words;
#[path = "user_buffer_tests.rs"]
mod user_buffer_tests;
#[path = "usercopy_tests.rs"]
mod usercopy_tests;
/// Which core services init restarts when they end.
#[path = "../../../src/userspace/init/supervisor/watch_rule.rs"]
pub mod watch_rule;
