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

//! Whether the kernel holds anything of the medium yet, which decides what
//! a change means (scan/check.rs), and the failed requests in a row the
//! device was kept after since it last moved the kernel's data, which
//! decide how long it is kept (scan/keep.rs).

use super::types::State;

impl State {
    /// A READ or WRITE of the kernel's was served. It is kept for the boot,
    /// whatever device is bound after: what the kernel read decided its disk.
    /// The device moved data, so its failures in a row start over; a flush
    /// moves none, and a stick that fails every READ may still take one.
    pub fn hold(&mut self) {
        (self.held, self.failed) = (true, 0);
    }

    /// Whether the kernel has read or written a sector this boot.
    pub fn held(&self) -> bool {
        self.held
    }

    /// One more failed request the check kept the device after, and how
    /// many in a row now. A device let go and bound again keeps its count:
    /// it has moved nothing since.
    pub fn kept_after_failure(&mut self) -> u8 {
        self.failed = self.failed.saturating_add(1);
        self.failed
    }
}
