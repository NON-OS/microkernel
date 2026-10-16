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

//! A controller as a bring-up leaves it when it gives up.

use std::sync::Arc;

use nonos_devmodel::{FakeBar, LiveDevice};

use crate::constants::{
    CORBCTL, CORBCTL_RUN, DPLBASE, DPLBASE_ENABLE, GCAP, GCTL, GCTL_CRST, INTCTL, RIRBCTL,
    RIRBCTL_DMAEN, RIRBCTL_RINTCTL, SDCTL_IOCE, SDCTL_RUN, SD_CTL,
};
use crate::model::{live, window};

/// Four input and four output engines: GCAP 0x4401.
pub const STREAMS: usize = 8;

pub fn sd_ctl(stream: usize) -> usize {
    0x80 + stream * 0x20 + SD_CTL as usize
}

/// Out of reset, both rings, the position buffer and an output engine on.
pub fn running() -> Arc<FakeBar> {
    let bar = window();
    bar.present16(GCAP as usize, 0x4401);
    bar.present32(GCTL as usize, GCTL_CRST);
    bar.present8(CORBCTL as usize, CORBCTL_RUN);
    bar.present8(RIRBCTL as usize, RIRBCTL_DMAEN | RIRBCTL_RINTCTL);
    bar.present32(DPLBASE as usize, 0x0010_0000 | DPLBASE_ENABLE);
    bar.present8(sd_ctl(4), SDCTL_RUN | SDCTL_IOCE);
    bar.present32(INTCTL as usize, 0x8000_0010);
    bar
}

/// An engine that keeps running whatever the driver writes to its RUN bit.
fn stuck(bar: &Arc<FakeBar>, stream: usize) -> LiveDevice {
    live(bar, move |b| {
        b.present8(sd_ctl(stream), b.wrote8(sd_ctl(stream)) | SDCTL_RUN);
        std::thread::yield_now();
    })
}

/// Run `f` on a running controller whose stream 4 never stops, until a run
/// in which the driver saw it held. The model is a thread, and on a loaded
/// test run it can miss the 5 ms a stream is given to stop; a run it missed
/// proves nothing either way, so `f` returns `None` for it.
pub fn with_stuck_engine<T>(mut f: impl FnMut(&Arc<FakeBar>) -> Option<T>) -> T {
    for _ in 0..20 {
        let bar = running();
        let _engine = stuck(&bar, 4);
        if let Some(t) = f(&bar) {
            return t;
        }
    }
    panic!("in 20 runs the driver never reported the engine that would not stop");
}
