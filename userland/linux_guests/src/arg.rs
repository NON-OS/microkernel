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

//! Buffers as syscall arguments.

pub fn p(b: &[u8]) -> u64 {
    b.as_ptr() as u64
}

pub fn pm(b: &mut [u8]) -> u64 {
    b.as_mut_ptr() as u64
}

pub fn pu(v: &mut u32) -> u64 {
    v as *mut u32 as u64
}
