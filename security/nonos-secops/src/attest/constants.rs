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

//! The attestation numbers the bootloader, the enroll tool and the capsule gate
//! agree on, held in one place for the security tools so a tool cannot drift from
//! the gate it tests. There are no proof parameters: the gate folds a path.

pub const DEPTH: usize = 8;
pub const LEAVES: usize = 1 << DEPTH;
pub const BOOT_EPOCH: u64 = 1;

/// The battery's own trees only. A release tree's pad seed comes from the enroll
/// tool's generator and is recorded in its transcript.
pub const PAD_SEED: [u8; 32] = [0x5a; 32];
