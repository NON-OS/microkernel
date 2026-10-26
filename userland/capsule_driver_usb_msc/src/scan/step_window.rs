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

//! How long one step of the scanner may hold the server (server/serve.rs).
//! While a device is served or being asked, the step talks to it, and what
//! waits behind it is bounded by the kernel's wait, so the step stays short.
//! While the scanner only lists the ports, a request is answered at once and
//! the step talks to driver.xhci0 alone, so it may outlast a CPU held by a
//! crowd of capsules starting on one core: driver.xhci0 then answers late,
//! not never, and a call that gave up first was reported as a server that
//! did not answer.

use super::medium::Medium;
use super::scanner::Scanner;

/// With a device served or asked: longer than driver.xhci0's own 5 s wait for a
/// transfer, so the controller says why one failed.
pub const STEP_MS: u64 = 6_000;
/// Listing the ports alone: as long as one call's own wait (xhci/call.rs, 12 s)
/// and more, so a call made as the CPU is held is answered in it.
pub const SEARCH_STEP_MS: u64 = 15_000;

impl Scanner {
    /// The window of the next step.
    pub fn step_ms(&self) -> u64 {
        let talks_to_a_device =
            self.asking.is_some() || matches!(self.medium, Medium::Ready(_) | Medium::Checking(_));
        if talks_to_a_device {
            STEP_MS
        } else {
            SEARCH_STEP_MS
        }
    }
}
