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

use crate::capabilities::{bit_of, Capability};

#[test]
fn capability_bits_are_distinct_single_bits() {
    // The gate ultimately rests on each capability occupying its own bit.
    let caps = [
        Capability::CoreExec,
        Capability::IO,
        Capability::Network,
        Capability::IPC,
        Capability::Memory,
        Capability::Crypto,
        Capability::FileSystem,
        Capability::Hardware,
        Capability::Debug,
        Capability::Admin,
    ];
    for c in caps {
        assert_eq!(bit_of(c).count_ones(), 1, "each capability must be a single bit");
    }
    for i in 0..caps.len() {
        for j in (i + 1)..caps.len() {
            assert_ne!(bit_of(caps[i]), bit_of(caps[j]), "two capabilities share a bit");
        }
    }
}
