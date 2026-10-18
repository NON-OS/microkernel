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

//! The names a package may not write: the files that say this machine
//! proved a program or vouched for its box. A package shipping one would be
//! installing its own proof, or its own policy.

/// Suffixes of the proof files beside a program, and of its vouched policy
/// (app/bind.rs reads both of those).
pub const REFUSED: [&[u8]; 5] = [
    b".nonos_id_cert.bin",
    b".manifest.bin",
    b".zk_trailer.bin",
    b".nonos_linux_policy.bin",
    b".nonos_linux_policy.zk_trailer.bin",
];

/// Whether `name` is one of them, wherever in the package it sits.
pub fn is_proof_name(name: &[u8]) -> bool {
    REFUSED.iter().any(|s| name.ends_with(s))
}
