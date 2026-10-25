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
//! The combo PHY's group addresses (Vol 12 p. 391): a write to one reaches
//! all four lanes, and a read of one is not to be trusted, so the model
//! answers it with junk that no correct read would act on. The port bases
//! and lane layout are stated here from Vol 2c's instance lists.

/// Port A, B and C's PHY blocks.
pub const BASES: [u32; 3] = [0x162000, 0x6C000, 0x160000];
/// What a read of a group address gives.
pub const JUNK: u32 = 0xDEAD_BEEF;

/// The four lane addresses a group address `a` writes, if it is one.
pub fn lanes(a: u32) -> Option<[u32; 4]> {
    let base = BASES.into_iter().find(|b| (b + 0x600..b + 0x700).contains(&a))?;
    let off = a - base - 0x600;
    Some([0, 1, 2, 3].map(|n| base + 0x800 + 0x100 * n + off))
}
