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

//! Who asked for a receipt, as the kernel recorded it: the measurement its
//! attestation registry holds for the asker's pid (MkAttestEntries, which
//! AttestRead opens to RECEIPT jobs alone). An entry is pid u32 BE |
//! measurement 32 | capability mask u64 | authority u8, as the kernel writes
//! it. The receipt names it as its launcher, under the receipt's challenge.
//! The pid is the asker's own: the job answers only a frame whose sender is
//! the pid init bound it to, so that process is alive as its entry is read,
//! and a live pid is never handed to another (process/core/table/pid.rs
//! counts up, wrapping only at u32::MAX, past every pid still in use).

pub const ENTRY_LEN: usize = 45;
/// Room for the whole registry: a buffer too small is refused outright.
pub const ENTRIES_MAX: usize = 256;

/// The measurement of `pid`'s entry, if the registry lists exactly one.
pub fn measure_of(entries: &[u8], pid: u32) -> Option<[u8; 32]> {
    let mut of = entries.chunks_exact(ENTRY_LEN).filter(|e| e[..4] == pid.to_be_bytes());
    let (Some(e), None) = (of.next(), of.next()) else { return None };
    e[4..36].try_into().ok()
}
