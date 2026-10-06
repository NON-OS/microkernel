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

/*
 * A syscall register is narrowed to its field or refused, never cut down.
 *
 * The kernel's narrowing rule is included by path. The microkernel calls took
 * 64-bit registers and cast them with `as u32`, so a pid of 2^32 + n reached
 * process n: MkKill, MkWait, MkPidAlive and the MkCap calls acted on a process
 * the caller never named, MkServiceRegister claimed a port it never asked
 * for, and an IPC endpoint of 2^32 + n resolved to port n. Device fields
 * were cut the same way: MkPciConfigWrite wrote the low 16 bits of a wider
 * value, MkPioGrant took BAR 256 as BAR 0, and the IRQ, DMA, device list,
 * vsync and policy calls dropped high bits of a flags word, a vector count,
 * a class or a field id. The checks below fail against that code: the rule
 * must round trip every value it admits, and the call sites must hold no
 * truncating cast left.
 */

#[path = "../../../../src/syscall/microkernel/narrow.rs"]
pub mod narrow;
mod tests;
