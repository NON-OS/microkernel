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

//! Host proofs, group 4 of 4: test modules mounted by path
//! from lib.rs, each at its own file.

#[path = "syscall_tests.rs"]
mod syscall_tests;
#[path = "uefi_cache/mod.rs"]
mod uefi_cache;
#[path = "usable_span_frame_tests.rs"]
mod usable_span_frame_tests;
#[path = "usable_span_tests.rs"]
mod usable_span_tests;
#[path = "user_buffer_tests.rs"]
mod user_buffer_tests;
#[path = "usercopy_tests.rs"]
mod usercopy_tests;
/// The restart policy init holds a watched service to.
#[path = "lifecycle_state.rs"]
pub mod lifecycle_state;
/// driver.usb_msc0's search report in words (usb_msc_report_tests).
#[path = "../../../src/hardware/usb_msc_capsule/report_words.rs"]
pub mod usb_msc_report_words;
/// Which core services init restarts when they end.
#[path = "../../../src/userspace/init/supervisor/watch_rule.rs"]
pub mod watch_rule;
/// The input ring's take clears what it hands out (K8).
#[path = "input_take/mod.rs"]
pub mod input_take;
/// An exiting process's surfaces refuse attaches (K15b).
#[path = "surface_closing/mod.rs"]
pub mod surface_closing;
/// Each input event's source, stamped by the kernel (K8b).
#[path = "input_source/mod.rs"]
pub mod input_source;
/// A capsule over the image ceiling is refused at spawn (K16).
#[path = "image_ceiling/mod.rs"]
pub mod image_ceiling;
/// A claimed function is reset at claim and at release (K10a).
#[path = "function_reset/mod.rs"]
pub mod function_reset;
/// IPC payloads zeroed before they are freed (C8).
#[path = "ipc_wipe/mod.rs"]
pub mod ipc_wipe;
