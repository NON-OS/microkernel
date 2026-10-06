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

//! Host proofs for nonos.prove: its pure half, the files that ship, fed what
//! the kernel and the registrar would hand it. The records come from the
//! kernel's own encoders over trees built the enroll tool's way, the registry
//! from `Registry` itself, so a layout one side writes and the other does not
//! read fails here rather than on the machine.

extern crate alloc;

/// libc's readers of the kernel's records, at the path the half imports.
pub mod abi;
/// The capsule's pure half, each file mounted where its imports expect.
pub mod assemble;
/// The kernel's `MkBootSlots` slot and record encoders, unchanged.
pub mod kernel_slots;

#[cfg(test)]
mod fixture;
/// libc's heap span, over a simulated \`MkMmap\`.
#[cfg(test)]
mod heap;
/// The kernel's \`MkMmap\` and \`MkMunmap\`, simulated, where libc's span
/// imports them.
#[cfg(test)]
mod mem;
#[cfg(test)]
mod tests;
