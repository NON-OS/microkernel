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

//! A registry entry, and the release the default host runs.

use nsh_attest::Release;

use super::spec::Entry;
use super::tcg_log::loader;

/// An entry under a developer key (authority byte 1).
pub fn entry(pid: u32, measurement: [u8; 32], caps: u64) -> Entry {
    entry_by(pid, measurement, caps, 1)
}

pub fn entry_by(pid: u32, measurement: [u8; 32], caps: u64, authority: u8) -> Entry {
    let mut e = [0u8; 45];
    e[..4].copy_from_slice(&pid.to_be_bytes());
    e[4..36].copy_from_slice(&measurement);
    e[36..44].copy_from_slice(&caps.to_be_bytes());
    e[44] = authority;
    e
}

pub fn release() -> Release {
    Release {
        version: "0.9.4".into(),
        pcr8: [0x08; 32],
        pcr9: [0x09; 32],
        loader: loader(),
        front: [0xF0; 32],
    }
}
