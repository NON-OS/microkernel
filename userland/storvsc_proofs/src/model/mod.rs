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

//! A Hyper-V storage host behind the driver's channel. Its control side is
//! nonos_vmbus's model of the host (the GPADL, the open and the close, the
//! clock); its storage side (`server`) is written from Linux's
//! storvsc_drv.c, which says what each request carries and what the host
//! answers, and nothing of the driver's code. The host reads and writes
//! the grant itself, by the page numbers it was handed, as Hyper-V does.

pub mod server;
#[path = "../../../nonos_scsi/src/sim.rs"]
pub mod sim;

use std::alloc::{alloc_zeroed, Layout as Alloc};

use nonos_vmbus::model::{Model, Script};
use nonos_vmbus::msg::MESSAGE_MAX;
use nonos_vmbus::{Host, Pages, PostError};

pub use server::{Server, Seen};

pub const PAGE: usize = 4096;
/// The page number the grant's first page has, as the host sees it.
pub const BASE_PFN: u64 = 0x4_0000;

/// A grant whose pages the driver and the host both reach, alive for the
/// thread.
#[derive(Clone, Copy)]
pub struct Grant {
    pub va: *mut u8,
    pub len: usize,
}

impl Grant {
    pub fn new(len: usize) -> Self {
        let layout = Alloc::from_size_align(len, PAGE).unwrap();
        // SAFETY: a non-zero, page-aligned layout; leaked for the test's life.
        Self { va: unsafe { alloc_zeroed(layout) }, len }
    }

    /// The first `len` bytes as shared pages.
    pub fn pages(&self, len: usize) -> Pages {
        assert!(len <= self.len);
        // SAFETY: inside the leaked grant.
        unsafe { Pages::new(self.va, len) }
    }

    /// The bytes at page number `pfn`, as the host reaches them.
    pub fn at_pfn(&self, pfn: u64) -> *mut u8 {
        let off = (pfn - BASE_PFN) as usize * PAGE;
        assert!(off + PAGE <= self.len, "a page outside the grant: {pfn:#x}");
        // SAFETY: checked inside the grant.
        unsafe { self.va.add(off) }
    }
}

/// The host: the control side and the storage side, the latter run each
/// time the driver signals or waits.
pub struct StorHost {
    pub vmbus: Model,
    pub server: Server,
    pub grant: Grant,
}

impl StorHost {
    pub fn new(grant: Grant, server: Server) -> Self {
        Self { vmbus: Model::new(Script::default()), server, grant }
    }

    fn run(&mut self) {
        if let Some((_, gpadl, in_at)) = self.vmbus.open {
            let pfns = self.vmbus.gpadls.iter().find(|(id, _)| *id == gpadl).map(|(_, p)| p.clone());
            if let Some(pfns) = pfns {
                self.server.run(self.grant, &pfns, in_at as usize);
            }
        }
    }
}

impl Host for StorHost {
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
