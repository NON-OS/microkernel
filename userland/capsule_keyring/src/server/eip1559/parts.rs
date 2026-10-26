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
//! The words a fixed-shape transaction is signed over, named once for the
//! signed and unsigned builders that take them.

/// nonce, tip, fee cap, gas, amount.
pub type StakeParts<'a> = (&'a [u8; 32], &'a [u8; 32], &'a [u8; 32], &'a [u8; 32], &'a [u8; 32]);
/// nonce, tip, fee cap, gas, amount, lock.
pub type LockedParts<'a> =
    (&'a [u8; 32], &'a [u8; 32], &'a [u8; 32], &'a [u8; 32], &'a [u8; 32], &'a [u8; 32]);
