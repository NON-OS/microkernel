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

//! `mk_dma_map`, answering from a script: each call takes the next grant the
//! test lined up (a refusal once they run out) and records the flags it was
//! asked with, so a test can place a grant across 4 GiB and see what the
//! driver asked for.

use std::cell::RefCell;

pub const MK_DMA_MAP_DMA32: u32 = 1 << 1;

#[derive(Clone, Copy, Debug, Default)]
pub struct DmaMapOut {
    pub user_va: u64,
    pub device_addr: u64,
    pub length: u64,
    pub grant_id: u64,
}

thread_local! {
    static GRANTS: RefCell<Vec<DmaMapOut>> = const { RefCell::new(Vec::new()) };
    static FLAGS: RefCell<Vec<u32>> = const { RefCell::new(Vec::new()) };
}

/// The grants the next calls answer with, in order.
pub fn line_up(grants: &[DmaMapOut]) {
    GRANTS.with(|g| *g.borrow_mut() = grants.iter().rev().copied().collect());
    FLAGS.with(|f| f.borrow_mut().clear());
}

/// The flags of every map asked for since `line_up`.
pub fn dma_flags() -> Vec<u32> {
    FLAGS.with(|f| f.borrow().clone())
}

pub fn mk_dma_map(_dev: u64, _epoch: u64, _len: u64, flags: u32, out: &mut DmaMapOut) -> i64 {
    FLAGS.with(|f| f.borrow_mut().push(flags));
    match GRANTS.with(|g| g.borrow_mut().pop()) {
        Some(grant) => {
            *out = grant;
            0
        }
        None => -12,
    }
}
