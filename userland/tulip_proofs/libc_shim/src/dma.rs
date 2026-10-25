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

//! `mk_dma_map` answering from a script: each call takes the next entry, a
//! device address or a negative errno, and records the flags it was asked.

use std::cell::RefCell;

pub const MK_DMA_MAP_DMA32: u32 = 1 << 1;
pub const MK_DMA_MAP_COHERENT: u32 = 1 << 2;

#[repr(C)]
pub struct DmaMapOut {
    pub user_va: u64,
    pub device_addr: u64,
    pub length: u64,
    pub grant_id: u64,
}

thread_local! {
    static SCRIPT: RefCell<Vec<Result<u64, i64>>> = const { RefCell::new(Vec::new()) };
    static ASKED: RefCell<Vec<u32>> = const { RefCell::new(Vec::new()) };
}

pub fn script_dma(answers: Vec<Result<u64, i64>>) {
    SCRIPT.with(|s| *s.borrow_mut() = answers.into_iter().rev().collect());
}

pub fn dma_flags_asked() -> Vec<u32> {
    ASKED.with(|a| a.borrow().clone())
}

pub fn mk_dma_map(_dev: u64, _epoch: u64, length: u64, flags: u32, out: *mut DmaMapOut) -> i64 {
    ASKED.with(|a| a.borrow_mut().push(flags));
    let next = SCRIPT.with(|s| s.borrow_mut().pop()).unwrap_or(Err(-12));
    match next {
        Ok(addr) => {
            let grant_id = ASKED.with(|a| a.borrow().len()) as u64;
            // SAFETY: the caller hands a writable DmaMapOut, as the real call requires.
            unsafe {
                *out = DmaMapOut { user_va: 0x7000_0000, device_addr: addr, length, grant_id }
            };
            0
        }
        Err(e) => e,
    }
}
