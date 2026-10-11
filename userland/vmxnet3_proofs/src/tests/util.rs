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

//! What several tests need: the address the shim's entropy draws, a driver
//! brought up on a fresh bench, a pool slot's bytes, the commands issued.

use crate::bench::Bench;
use crate::constants::ring::BUFFER_LEN;
use crate::init::finish;
use crate::model::Event;
use crate::setup::Driver;

pub fn drawn() -> [u8; 6] {
    let mut mac: [u8; 6] = core::array::from_fn(|i| 0xC3 ^ i as u8);
    nonos_mac::apply(&mut mac);
    mac
}

pub fn up(bench: &mut Bench) -> Driver {
    finish(bench.with(|_| {})).expect("bring-up on a well-behaved device")
}

pub fn slot(pool_va: u64, idx: u16) -> &'static mut [u8] {
    // SAFETY: the bench's pools outlive every test that asks, and a slot is
    // BUFFER_LEN bytes inside one.
    unsafe {
        std::slice::from_raw_parts_mut(
            (pool_va + idx as u64 * BUFFER_LEN as u64) as *mut u8,
            BUFFER_LEN,
        )
    }
}

pub fn commands(log: &[Event]) -> Vec<u32> {
    log.iter()
        .filter_map(|e| if let Event::Command { cmd, .. } = e { Some(*cmd) } else { None })
        .collect()
}
