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

//! The loopback's randomness, the system's, and its clock, moving as real
//! time does from the role table's or another.

use crate::kit::roles_build::NOW;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

/// The system's randomness.
pub fn urandom(out: &mut [u8]) -> bool {
    File::open("/dev/urandom").and_then(|mut f| f.read_exact(out)).is_ok()
}

/// A clock moving as real time does, from where it was started.
pub struct Clock {
    started: Instant,
    base_ms: u64,
}

impl Clock {
    /// The role table's clock.
    pub fn start() -> Clock {
        Clock::at(NOW)
    }
    pub fn at(base_ms: u64) -> Clock {
        Clock { started: Instant::now(), base_ms }
    }
    pub fn now(&self) -> u64 {
        self.base_ms + self.started.elapsed().as_millis() as u64
    }
}
