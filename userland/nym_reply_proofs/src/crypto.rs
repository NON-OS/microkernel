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

//! What the session table needs of net.nym's crypto: the session key type,
//! as `crypto/types.rs` defines it, and a random source. The table never
//! reads the bytes it draws, so a fixed fill stands in for the kernel's.

/// A session key, `crypto/types.rs`'s `Key`.
pub type Key = [u8; 32];

pub mod random {
    /// The source failed; never, here.
    pub struct Unavailable;

    /// Fill `out`; the kernel's source on the device, a fixed byte here.
    pub fn fill_random(out: &mut [u8]) -> Result<(), Unavailable> {
        out.fill(0x5A);
        Ok(())
    }
}
