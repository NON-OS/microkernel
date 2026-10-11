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

//! A package's files going, each with what the machine wrote beside it: the
//! trailer it minted, and for a market program the policy it vouched and that
//! policy's trailer (`market`), so nothing it vouched outlives the program.

use alloc::vec::Vec;

use crate::linux::attest_paths::beside;
use crate::linux::file::{key, store_stat, store_unlink};

const MINTED: [&[u8]; 3] =
    [b".zk_trailer.bin", b".nonos_linux_policy.bin", b".nonos_linux_policy.zk_trailer.bin"];

/// The files that would not go; gone already is as good as taken out.
pub(super) fn take_out(files: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let mut left = Vec::new();
    for at in files {
        if store_unlink(&key(at)).is_err() && store_stat(&key(at)).is_ok() {
            left.push(at.clone());
        }
        for suffix in MINTED {
            let _ = store_unlink(&key(&beside(at, suffix)));
        }
    }
    left
}
