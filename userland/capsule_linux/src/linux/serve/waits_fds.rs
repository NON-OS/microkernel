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

//! The descriptors a parked wait watches, read from the call's own list.

use alloc::vec::Vec;

use crate::linux::abi::{nr, nr_path as np};
use crate::linux::guest::{Blocked, Guest};

/// More than a guest can have open, so a longer list is read no further.
const MOST: u64 = 256;

pub fn watched(guest: &Guest, wait: &Blocked) -> Vec<u64> {
    let a = wait.args;
    match wait.nr {
        nr::READ | nr::WRITE => alloc::vec![a[0]],
        nr::POLL | np::PPOLL => (0..a[1].min(MOST))
            .filter_map(|i| guest.read(a[0] + i * 8, 4))
            .map(|raw| u64::from(u32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]])))
            .collect(),
        np::SELECT | np::PSELECT6 => [a[1], a[2]]
            .into_iter()
            .filter(|&at| at != 0)
            .filter_map(|at| guest.read(at, a[0].min(MOST).div_ceil(8) as usize))
            .flat_map(|set| {
                (0..a[0].min(MOST)).filter(move |&fd| set[(fd / 8) as usize] & (1 << (fd % 8)) != 0)
            })
            .collect(),
        _ => guest
            .fds
            .get(a[0] as usize)
            .map_or(Vec::new(), |l| l.watch.iter().map(|w| w.fd).collect()),
    }
}
