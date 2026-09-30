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

//! The measurement a capsule is attested under: the BLAKE3 digest of its ELF.
//!
//! Taken once per spawn, before any root is tried, so a capsule that falls
//! through to the enrolled roots is not hashed again for each of them. An
//! image is megabytes and a spawn from a system call runs with interrupts
//! masked, so the digest is taken a serve unit at a time, answering TLB
//! shootdowns in between. The pieces are fed to one hasher in order, so the
//! digest is exactly `blake3::hash(elf)`.

pub(super) fn measure(elf: &[u8]) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    crate::smp::in_serve_units(elf, |piece| {
        hasher.update(piece);
    });
    *hasher.finalize().as_bytes()
}
