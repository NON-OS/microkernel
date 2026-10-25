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

// The weak paths the callers must not use, included into `rng`, and the
// CPU's random instruction, absent here so mixing it in changes nothing.

/// The kernel's weak fill: it never refuses. Its bytes are a fixed fill.
pub fn fill_random_bytes(buf: &mut [u8]) {
    note_weak();
    buf.fill(0xa5);
}

/// The weak fill as `crypto::rng::fill_bytes` names it.
pub fn fill_bytes(buf: &mut [u8]) {
    fill_random_bytes(buf);
}

/// The weak word, which the erase seeds use on purpose.
pub fn random_u64() -> u64 {
    note_weak();
    0x5a5a_0000_0000_0000 | WEAK.get()
}

/// A weak source was called: these, or the cycle counter as a seed.
pub(crate) fn note_weak() {
    WEAK.set(WEAK.get() + 1);
}

pub fn has_cpu_random() -> bool {
    false
}
pub fn has_cpu_entropy() -> bool {
    false
}
pub fn cpu_entropy64() -> Option<u64> {
    None
}
pub fn cpu_random64() -> Option<u64> {
    None
}
