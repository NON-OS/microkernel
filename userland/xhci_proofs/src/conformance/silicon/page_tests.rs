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

//! The page size register, scratchpad alignment, 32-bit-only DMA, and the
//! wait after HCRST.

use nonos_devmodel::run;

use super::super::devices::host_controller;
use super::super::model::{controller, op_base};
use crate::controller::bringup::scratchpad::aligned_page;
use crate::controller::{reset, Scratchpads};
use crate::dma::DmaPool;
use crate::regs::op::page_bytes;

#[test]
fn reset_waits_after_hcrst_before_reading_the_controller() {
    let bar = controller(0);
    let _hc = run(&bar, host_controller);
    let before = nonos_libc::slept_ms();
    reset(op_base(&bar)).expect("reset completes");
    assert!(nonos_libc::slept_ms() > before, "no settle time after HCRST (Intel quirk)");
}

#[test]
fn the_page_size_register_is_read_as_its_lowest_bit() {
    assert_eq!(page_bytes(0x1), Some(4096));
    assert_eq!(page_bytes(0x2), Some(8192));
    assert_eq!(page_bytes(0x6), Some(8192), "the smallest of several");
    assert_eq!(page_bytes(0), None);
    assert_eq!(page_bytes(0xFFFF_0000), None, "bits 31:16 are reserved");
}

#[test]
fn scratchpads_are_aligned_to_the_controller_page_size() {
    let pool = DmaPool::new(1, 1);
    for page in [4096u64, 8192, 32768] {
        let pads = Scratchpads::allocate(&pool, 3, page).expect("scratchpads");
        assert_eq!(pads.page_count(), 3);
        let array = nonos_libc::dma_host(pads.array_phys()).expect("array mapped") as *const u64;
        for i in 0..3 {
            // SAFETY: the array holds three entries in a live grant.
            let entry = unsafe { array.add(i).read() };
            assert_eq!(entry % page, 0, "entry {i} not aligned to {page}");
            assert!(nonos_libc::dma_host(entry + page - 1).is_some(), "page {i} not whole");
        }
    }
    assert!(Scratchpads::allocate(&pool, 1, 6000).is_err(), "not a power of two");
    assert_eq!(aligned_page(0x1000, 0x2000), 0x2000);
    assert_eq!(aligned_page(0x4000, 0x2000), 0x4000);
}
