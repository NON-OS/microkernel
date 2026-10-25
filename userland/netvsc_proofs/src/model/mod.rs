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

//! A Hyper-V network host behind the driver's channel. Its control side is
//! nonos_vmbus's model of the host (the GPADLs, the open and the close, the
//! clock); its network side (`netvsp`) is written from OpenVMM's netvsp, the
//! host side Microsoft publishes, and nothing of the driver's code; past it
//! sits a switch with a DHCP server (`wire`). The host reads and writes the
//! grants itself, by the page numbers it was handed, as Hyper-V does.

pub mod netvsp;
pub mod wire;

use std::alloc::{alloc_zeroed, Layout as Alloc};

use nonos_vmbus::model::{Model, Script};
use nonos_vmbus::msg::MESSAGE_MAX;
use nonos_vmbus::{Host, Pages, PostError};

pub use netvsp::Netvsp;

pub const PAGE: usize = 4096;
/// The page numbers of the three grants, as the host sees them.
pub const RINGS_PFN: u64 = 0x4_0000;
pub const RECV_PFN: u64 = RINGS_PFN + 64;
pub const SEND_PFN: u64 = RINGS_PFN + 128;
const PAGES: usize = 192;

/// The three grants in one allocation, alive for the thread.
#[derive(Clone, Copy)]
pub struct Grants {
    pub va: *mut u8,
}

impl Grants {
    pub fn new() -> Self {
        let layout = Alloc::from_size_align(PAGES * PAGE, PAGE).unwrap();
        // SAFETY: a non-zero, page-aligned layout; leaked for the test's life.
        Self { va: unsafe { alloc_zeroed(layout) } }
    }

    /// The 64 pages from page number `pfn`, as shared pages.
    pub fn pages(&self, pfn: u64) -> Pages {
        let off = (pfn - RINGS_PFN) as usize * PAGE;
        assert!(off + 64 * PAGE <= PAGES * PAGE);
        // SAFETY: inside the leaked allocation.
        unsafe { Pages::new(self.va.add(off), 64 * PAGE) }
    }

    /// The byte at offset `at` of the buffer whose pages are `pfns`.
    pub fn at(&self, pfns: &[u64], at: usize) -> *mut u8 {
        let pfn = pfns[at / PAGE];
        let off = (pfn - RINGS_PFN) as usize * PAGE + at % PAGE;
        assert!(off < PAGES * PAGE, "a page outside the grants: {pfn:#x}");
        // SAFETY: checked inside the allocation.
        unsafe { self.va.add(off) }
    }

    pub fn read(&self, pfns: &[u64], at: usize, len: usize) -> Vec<u8> {
        // SAFETY: each byte through `at`, which checks it.
        (0..len).map(|i| unsafe { *self.at(pfns, at + i) }).collect()
    }

    pub fn write(&self, pfns: &[u64], at: usize, data: &[u8]) {
        for (i, b) in data.iter().enumerate() {
            // SAFETY: as above.
            unsafe { *self.at(pfns, at + i) = *b };
        }
    }
}

impl Default for Grants {
    fn default() -> Self {
        Self::new()
    }
}

/// The host: the control side and the network side, the latter run each
/// time the driver signals or waits.
pub struct NetHost {
    pub vmbus: Model,
    pub netvsp: Netvsp,
}

impl NetHost {
    pub fn new(netvsp: Netvsp) -> Self {
        Self { vmbus: Model::new(Script::default()), netvsp }
    }

    fn run(&mut self) {
        if let Some((_, gpadl, in_at)) = self.vmbus.open {
            let gpadls = self.vmbus.gpadls.clone();
            self.netvsp.run(&gpadls, gpadl, in_at as usize, self.vmbus.now);
        }
    }
}

impl Host for NetHost {
    fn post(&mut self, m: &[u8]) -> Result<(), PostError> {
        self.vmbus.post(m)
    }

    fn answer(&mut self, out: &mut [u8; MESSAGE_MAX]) -> Option<usize> {
        self.vmbus.answer(out)
    }

    fn signal(&mut self) {
        self.vmbus.signal();
        self.run();
    }

    fn now_ms(&mut self) -> u64 {
        self.vmbus.now_ms()
    }

    fn idle(&mut self) {
        self.vmbus.idle();
        self.run();
    }
}
