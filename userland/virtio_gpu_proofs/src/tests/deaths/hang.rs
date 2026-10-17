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
//! The model's step hung on the driver's yield, and a rig's end: the hook
//! taken off and every DMA map of the thread given back.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use nonos_devmodel::FakeBar;
use nonos_libc::{dma_reset, off_yield, on_yield};

use super::rig::Rig;
use crate::tests::host::gpu::Gpu;

impl Drop for Rig {
    fn drop(&mut self) {
        off_yield();
        dma_reset();
    }
}

/// The model's step hung on this thread's yield.
pub fn hang(gpu: &Rc<RefCell<Gpu>>, region: Arc<FakeBar>) {
    let gpu = Rc::clone(gpu);
    on_yield(move || {
        let killed = gpu.borrow_mut().serve(&region);
        if killed {
            panic!("the driver is killed");
        }
    });
}
