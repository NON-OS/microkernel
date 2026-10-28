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

//! The broker's DMA grant as the setup's dma.rs asks for it: the length a
//! test chooses (the length asked when it chooses none), and every grant
//! given back recorded.

use std::cell::{Cell, RefCell};

pub const MK_DMA_MAP_COHERENT: u32 = 1;
pub const GRANT: u64 = 7;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct DmaMapOut {
    pub user_va: u64,
    pub device_addr: u64,
    pub length: u64,
    pub grant_id: u64,
}

thread_local! {
    static LENGTH: Cell<Option<u64>> = const { Cell::new(None) };
    static UNMAPPED: RefCell<Vec<u64>> = const { RefCell::new(Vec::new()) };
}

/// The length the next grants come back with.
pub fn grant_length(n: Option<u64>) {
    LENGTH.with(|l| l.set(n));
}

/// The grants given back since the last call.
pub fn unmapped() -> Vec<u64> {
    UNMAPPED.with(|u| std::mem::take(&mut *u.borrow_mut()))
}

pub fn mk_dma_map(_dev: u64, _epoch: u64, len: u64, _flags: u32, out: &mut DmaMapOut) -> i64 {
    let length = LENGTH.with(|l| l.get()).unwrap_or(len);
    *out = DmaMapOut { user_va: 0x10_0000, device_addr: 0x20_0000, length, grant_id: GRANT };
    0
}

pub fn mk_dma_unmap(grant_id: u64) -> i64 {
    UNMAPPED.with(|u| u.borrow_mut().push(grant_id));
    0
}
