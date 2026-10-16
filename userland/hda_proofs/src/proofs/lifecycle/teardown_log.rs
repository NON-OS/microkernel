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

//! A teardown, run and read back: the broker calls it made, each with the
//! registers and the PCI command word as they stood when it was made.

use std::sync::Arc;

use nonos_devmodel::FakeBar;
use nonos_libc::{calls, Call, MmioMapOut};

use crate::constants::{CORBCTL, DPLBASE, GCTL, RIRBCTL};
use crate::handles::BrokerHandles;
use crate::proofs::lifecycle::teardown::{sd_ctl, STREAMS};

/// Where each register sits in a call's `regs`, after the streams' SDnCTL.
pub const CORB: usize = STREAMS;
pub const RIRB: usize = STREAMS + 1;
pub const DPL: usize = STREAMS + 2;
pub const GCTL_AT: usize = STREAMS + 3;
pub const CMD: usize = STREAMS + 4;
const DEVICE: u64 = 0x5a;

pub fn snapshot(bar: &Arc<FakeBar>) -> impl Fn() -> Vec<u32> + 'static {
    let bar = Arc::clone(bar);
    move || {
        let mut v: Vec<u32> = (0..STREAMS).map(|s| bar.wrote8(sd_ctl(s)) as u32).collect();
        v.push(bar.wrote8(CORBCTL as usize) as u32);
        v.push(bar.wrote8(RIRBCTL as usize) as u32);
        v.push(bar.wrote32(DPLBASE as usize));
        v.push(bar.wrote32(GCTL as usize));
        v.push(nonos_libc::command() as u32);
        v
    }
}

/// Drop the handles over `bar` and return the broker calls the drop made.
pub fn teardown(bar: &Arc<FakeBar>) -> Vec<Call> {
    nonos_libc::watch(snapshot(bar));
    let before = calls().len();
    let mmio = MmioMapOut { user_va: bar.base(), length: bar.len() as u64, grant_id: 0x100 };
    drop(BrokerHandles::new(DEVICE, 7, &mmio, 0x22));
    calls().split_off(before)
}

/// The release is the one call that gives grants back, and it comes last.
pub fn release(log: &[Call]) -> &Call {
    let last = log.last().expect("the teardown made no broker call");
    assert_eq!((last.name, last.arg), ("device_release", DEVICE), "{log:?}");
    assert_eq!(log.iter().filter(|c| c.name == "device_release").count(), 1);
    last
}
