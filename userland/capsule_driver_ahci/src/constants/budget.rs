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

//! The longest one request can hold the port, held at build time under the
//! kernel's wait for its reply. Past that wait the kernel reports the
//! request lost while the driver may still be working on it.

use super::timing::{
    BRING_UP_COMMAND_MS, COMMAND_MS, COMRESET_HOLD_MS, ENGINE_STOP_MS, LINK_TIMEOUT_MS, READY_MS,
    RECOVER_READY_MS,
};

/// The kernel's wait for a reply (src/services/lifecycle/reply_wait.rs,
/// SLOW_BUDGET_MS).
pub const KERNEL_REPLY_MS: u64 = 35_000;

/// The longest one recovery takes: the engine stop; a second stop after the
/// COMRESET that frees a stuck CR; the start; and one COMRESET with its
/// hold, the link's return and the device's ready.
pub const RECOVER_MS: u64 =
    3 * ENGINE_STOP_MS + COMRESET_HOLD_MS + LINK_TIMEOUT_MS + RECOVER_READY_MS;

/// The longest the wait before a command takes: READY_MS, one recovery,
/// and READY_MS again (a drive still in its reset gets READY_MS alone). It
/// comes out of the command's budget.
pub const PRE_ISSUE_MS: u64 = 2 * READY_MS + RECOVER_MS;

/// The least time an issued command is ever given: its budget, less the
/// longest wait before it.
pub const MIN_COMMAND_MS: u64 = BRING_UP_COMMAND_MS - PRE_ISSUE_MS;

/// The longest one request takes: its budget, and the recovery after the
/// command failed.
pub const REQUEST_MS: u64 = COMMAND_MS + RECOVER_MS;

const _: () = assert!(REQUEST_MS < KERNEL_REPLY_MS);
const _: () = assert!(PRE_ISSUE_MS < BRING_UP_COMMAND_MS && BRING_UP_COMMAND_MS <= COMMAND_MS);
const _: () = assert!(MIN_COMMAND_MS >= 3_000);
