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

//! One bring-up attempt with every broker call it gives back recorded, and
//! the order those calls must come in.

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use nonos_devmodel::run;
use nonos_libc::{offer_mmio, start_log, take_log, Call, Logged, MK_PCI_CMD_BUS_MASTER};

use super::bringup_device::{controller_that, Once};
use super::bringup_model::{bringup_controller, found, CMD, STS, WINDOW};
use crate::error::XhciError;

/// One attempt against a controller that behaves as `once` says; its
/// result and every broker call it gave back.
pub fn attempt(once: Once) -> (Result<(), XhciError>, Vec<Logged>) {
    let bar = bringup_controller();
    let resets = Arc::new(AtomicU32::new(0));
    let _hc = run(&bar, controller_that(once, resets.clone(), Default::default()));
    offer_mmio(bar.base(), WINDOW as u64);
    let b = bar.clone();
    start_log(move || {
        let regs = (b.wrote32(STS) as u64) << 32 | b.wrote32(CMD) as u64;
        (resets.load(Ordering::SeqCst) as u64) << 48 | regs
    });
    let got = crate::setup::run(found()).map(|_| ());
    (got, take_log())
}

/// Where bus mastering went off again after bring-up turned it on, what
/// the probe saw then, and the order.
pub fn stopped_then_released(log: &[Logged]) -> u64 {
    let calls: Vec<Call> = log.iter().map(|l| l.call).collect();
    let release = calls.iter().position(|&c| c == Call::DeviceRelease(7)).expect("released");
    let freed = calls.iter().position(|c| matches!(c, Call::DmaUnmap(_))).expect("DMA freed");
    assert!(release < freed, "the device is released before any DMA frame: {calls:?}");
    let bus_master = |c: &Call| matches!(c, Call::PciCommand(v) if v & MK_PCI_CMD_BUS_MASTER != 0);
    let on = calls.iter().position(bus_master).expect("bus mastering turned on");
    let is_off = |c: &Call| matches!(c, Call::PciCommand(_)) && !bus_master(c);
    let off = on + calls[on..].iter().position(is_off).expect("bus mastering turned off");
    assert!(off < release, "bus mastering off before the release: {calls:?}");
    log[off].seen
}
