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

//! The kernel's hardware-generator fallback (`try_fill_random`), served from
//! the host's random source.

use std::io::Read;

pub(super) fn host_random(out: &mut [u8]) -> bool {
    std::fs::File::open("/dev/urandom").and_then(|mut f| f.read_exact(out)).is_ok()
}

pub mod random {
    pub fn try_fill_random(buf: &mut [u8]) -> Result<(), &'static str> {
        if super::host_random(buf) {
            Ok(())
        } else {
            Err("no host random source")
        }
    }
}
