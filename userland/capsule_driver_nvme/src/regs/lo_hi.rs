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

//! 64-bit NVMe registers (CAP, ASQ, ACQ) moved as two 32-bit accesses, low
//! dword first, the way Linux's lo_hi_readq and lo_hi_writeq do. A single
//! 8-byte MMIO access is split or refused by some PCIe root complexes and
//! controllers, and a bridge that splits it is free to send the halves in
//! either order; NVMe only promises that both 32-bit halves are accessible.
//! Pure, so the host proofs check the split and the join.

/// The (low, high) dwords of `value`, in the order they are written.
pub const fn split_lo_hi(value: u64) -> (u32, u32) {
    (value as u32, (value >> 32) as u32)
}

/// The 64-bit value read back as a low dword and then a high one.
pub const fn join_lo_hi(lo: u32, hi: u32) -> u64 {
    ((hi as u64) << 32) | lo as u64
}
