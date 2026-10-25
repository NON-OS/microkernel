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

//! The operating system's randomness, for key pairs and ML-DSA signing.
//! A short or failed read refuses: nothing is made from zeros.

use std::fs::File;
use std::io::Read;

/// Fills `out` from the system's random device; `false` if it could not.
pub fn fill(out: &mut [u8]) -> bool {
    File::open("/dev/urandom").and_then(|mut f| f.read_exact(out)).is_ok()
}

/// A source in the shape the ML-DSA crate takes.
pub fn source() -> impl FnMut(&mut [u8]) -> bool {
    fill
}
