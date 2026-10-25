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

//! The image ceiling's rule, pure (K16): a capsule whose installed bits
//! go past what the image was built to grant is refused at spawn, and the
//! refusal names the bits. kernel_proofs::image_ceiling.

/// The bits of `requested` that `ceiling` does not admit.
pub(super) const fn over(requested: u64, ceiling: u64) -> u64 {
    requested & !ceiling
}

/// Ok when every requested bit is under the ceiling; else the bits over.
pub const fn gate(requested: u64, ceiling: u64) -> Result<(), u64> {
    match over(requested, ceiling) {
        0 => Ok(()),
        excess => Err(excess),
    }
}
