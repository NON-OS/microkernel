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

#[path = "call_wire.rs"]
mod call_wire;

use call_wire::call_within;

// Install reads the capsule's artifacts out of the store in chunks and runs
// the full verify and attestation before it replies, so it is inherently
// slower than an ordinary service call. A multi-megabyte binary can take a
// good few seconds; give the reply a wide budget rather than the 5s default,
// which a large capsule blows through while the load is still legitimately
// running.
const INSTALL_TIMEOUT_MS: u64 = 30_000;

// Ask the installer to load a store capsule by name. Only the name, caps and
// argv travel here; the installer reads the (multi-MB) artifacts from the
// store itself, so the message stays small. Returns the new pid, or a
// negative status.
pub(crate) fn call_installer(name: &[u8], args: &[u8]) -> Result<u32, i32> {
    call_within(name, args, INSTALL_TIMEOUT_MS)
}

/// How a load request went within the short wait `install` gives it.
pub(super) enum Issued {
    Loaded(u32),
    Refused(i32),
    /// Delivered, and the installer is still at it (`follow.rs`).
    InFlight,
}

/// Wait only briefly: a quick answer (a refusal, a missing name) is taken
/// at once, and a load still running is followed from the job's ticks.
const ISSUE_TIMEOUT_MS: u64 = 40;
const ETIMEDOUT: i32 = -110;

pub(super) fn issue(name: &[u8], args: &[u8]) -> Issued {
    match call_within(name, args, ISSUE_TIMEOUT_MS) {
        Ok(pid) => Issued::Loaded(pid),
        Err(ETIMEDOUT) => Issued::InFlight,
        Err(status) => Issued::Refused(status),
    }
}
