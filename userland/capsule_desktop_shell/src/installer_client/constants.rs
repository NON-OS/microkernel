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

//! Wire constants for the installer protocol.

pub(super) const SERVICE: &[u8] = b"installer";
pub(super) const HDR_LEN: usize = 8;
pub(super) const SEQ: u32 = 1;

pub(super) const OP_LIST_INSTALLED: u16 = 5;
pub(super) const OP_LOAD_BY_NAME: u16 = 4;
pub(super) const OP_PKG_QUERY: u16 = 6;
pub(super) const OP_PKG_COMMIT: u16 = 7;
/// How this shell's last load of a name ended.
pub(super) const OP_LOAD_OUTCOME: u16 = 9;

pub(super) const TIMEOUT_MS: u64 = 300;

/// Loading reads the capsule's artifacts out of the store and runs the full
/// verify and attestation before the installer replies, so a multi-megabyte
/// binary legitimately takes seconds. The listing budget would abandon a load
/// that is still running correctly.
pub(super) const LOAD_TIMEOUT_MS: u64 = 30_000;

/// A Launchpad launch does not wait a load out on the shell's frame loop: it
/// waits this long for a quick answer (a refusal, a missing name) and follows
/// a load still running on later turns (`state/launch.rs`).
pub(super) const ISSUE_TIMEOUT_MS: u64 = 40;

/// A health probe the installer answers at once when it is free. Busy with a
/// load, it answers once the load is done, long after this.
pub(super) const PROBE_TIMEOUT_MS: u64 = 20;

/// None named: the installer asks for the app's own manifest set, and the
/// kernel refuses a request beyond the manifest. The shell never picks what
/// an app it launches holds; SpawnWindow lets that exceed the shell's own.
pub(super) const REQUESTED_CAPS: u64 = 0;

/// Reply buffer size, the same budget the desktop gives a vfs listing. Anything
/// past it is simply not shown rather than risking a large per-tick allocation.
pub(super) const REPLY_CAP: usize = 16384;

/// Longest capsule-store base name the store contract allows.
pub(super) const MAX_NAME: usize = 64;

/// Upper bound on decoded names, so a hostile reply cannot turn one buffer into
/// an unbounded number of allocations.
pub(super) const MAX_ENTRIES: usize = 256;
