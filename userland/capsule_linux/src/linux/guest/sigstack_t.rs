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

//! Linux's `stack_t`, the form `sigaltstack` reads and writes a stack in.

use super::sigstack::{AltStack, SS_DISABLE, SS_ONSTACK};

/// `stack_t` is 24 bytes: ss_sp, ss_flags (an int, then 4 bytes of padding),
/// ss_size.
pub const STACK_T: usize = 24;
/// Linux 4.7's flag bit that disarms the stack while a handler runs on it.
pub const SS_AUTODISARM: u32 = 1 << 31;

/// The base, flags and size a `stack_t` holds.
pub fn decode(raw: &[u8]) -> (u64, u32, u64) {
    let word = |at: usize| u64::from_le_bytes(raw[at..at + 8].try_into().unwrap_or([0; 8]));
    let flags = u32::from_le_bytes(raw[8..12].try_into().unwrap_or([0; 4]));
    (word(0), flags, word(16))
}

/// A `stack_t` for `stack`, its flags as Linux's `sas_ss_flags` gives them.
pub fn encode(stack: Option<AltStack>, rsp: u64) -> [u8; STACK_T] {
    let (sp, size, flags) = match stack {
        None => (0, 0, SS_DISABLE),
        Some(s) if s.holds(rsp) => (s.sp, s.size, SS_ONSTACK),
        Some(s) => (s.sp, s.size, 0),
    };
    let mut out = [0u8; STACK_T];
    out[0..8].copy_from_slice(&sp.to_le_bytes());
    out[8..12].copy_from_slice(&flags.to_le_bytes());
    out[16..24].copy_from_slice(&size.to_le_bytes());
    out
}
