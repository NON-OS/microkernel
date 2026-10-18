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

//! How the kernel's seal counts calls (plan/10): bucket b is call number b
//! below 512, and every larger number shares bucket 512.

pub const BUCKETS: usize = 513;

/// The bucket of call number `nr`, as the kernel counts it.
pub const fn bucket(nr: u64) -> usize {
    if nr < 512 {
        nr as usize
    } else {
        512
    }
}
