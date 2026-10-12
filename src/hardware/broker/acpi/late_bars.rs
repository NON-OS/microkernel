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

//! Whether a PCI function found after the boot scan can be used as found.
//! Firmware assigns BARs while it enumerates; a function it left powered
//! off was skipped, and its BARs still read zero. NONOS does not assign
//! BARs, so such a function is recorded and marked unusable. Pure:
//! `kernel_proofs` mounts this file as it is.

/// `bars` holds each BAR's (size, base): a memory BAR's address or an I/O
/// BAR's port. A BAR with a size and a zero base was never assigned.
pub fn unassigned(bars: &[(u64, u64)]) -> bool {
    bars.iter().any(|&(size, base)| size != 0 && base == 0)
}
