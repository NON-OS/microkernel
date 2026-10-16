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

//! A virtio-rng device as the window shows it before the driver arrives,
//! and host memory handed to the driver the way a grant is.

use nonos_devmodel::FakeBar;
use nonos_virtio::Mmio;

use crate::constants::LEG_QUEUE_NUM;
use crate::init::{negotiate, start};
use crate::regs::Regs;

const WINDOW: usize = 256;

/// A device advertising a queue of `qmax` entries and nothing else.
pub fn device(qmax: u16) -> FakeBar {
    let bar = FakeBar::new(WINDOW);
    bar.present16(LEG_QUEUE_NUM, qmax);
    bar
}

/// All of `bar`, as the bounds-checked view the driver is handed.
pub fn view(bar: &FakeBar) -> Mmio {
    // SAFETY: the cells live as long as `bar`, and every test keeps `bar`
    // until it is done with the view.
    unsafe { Mmio::new(bar.base() as *mut u8, bar.len()) }
}

/// The driver's legacy register window over `bar`.
pub fn regs(bar: &FakeBar) -> Regs {
    Regs::mmio(view(bar))
}

/// The legacy handshake as `setup::legacy` runs it, without the grants
/// between its steps or a broker to turn bus mastering on.
pub fn handshake(regs: Regs, queue_phys: u64) -> Result<u16, &'static str> {
    negotiate(regs)?;
    start(regs, queue_phys, || Ok(()))
}
