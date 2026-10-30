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
//! An Intel PCH xHCI whose USB 2.0 and SuperSpeed ports firmware lets the
//! OS move over from EHCI, and one bring-up on it with every broker
//! call recorded, each with what the controller showed then.

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use nonos_devmodel::{run, LiveDevice};
use nonos_libc::{offer_config, offer_mmio, start_log, Call, Logged};

use super::bringup_device::{controller_that, Once};
use super::bringup_model::{bringup_controller, found, CMD, STS, WINDOW};
use crate::discover::Found;
use crate::error::XhciResult;
use crate::setup::Driver;

pub const SUBSYSTEM: u32 = 0x2C;
pub const XUSB2PR: u32 = 0xD0;
pub const USB3_PSSEN: u32 = 0xD8;
/// A Lenovo board's subsystem ids, and the Sony VAIO's that cannot switch.
pub const ANY_BOARD: u32 = 0x2210_17AA;
pub const VAIO: u32 = 0x90A8_104D;

/// A Lynx Point controller, Haswell's PCH.
pub fn lynx_point() -> Found {
    Found { vendor: 0x8086, device: 0x8C31, ..found() }
}

/// Config space as firmware left it: nothing routed, `usb3` and `usb2` the
/// masks it lets the OS set, admitted by the broker as it reads them.
pub fn pch(subsystem: u32, usb3: u16, usb2: u16, takes: bool) {
    let regs = [(SUBSYSTEM, subsystem), (XUSB2PR, 0), (0xD4, usb2 as u32)];
    let regs = [regs[0], regs[1], regs[2], (USB3_PSSEN, 0), (0xDC, usb3 as u32)];
    offer_config(&regs, &[(XUSB2PR, usb2), (USB3_PSSEN, usb3)], takes);
}

/// `dev` brought up against a controller behaving as `once` says, the log
/// started; the probe reads resets << 48 | USBSTS << 32 | USBCMD.
pub fn bring_up(dev: Found, once: Once) -> (LiveDevice, XhciResult<Driver>) {
    let bar = bringup_controller();
    let resets = Arc::new(AtomicU32::new(0));
    let hc = run(&bar, controller_that(once, resets.clone(), Default::default()));
    offer_mmio(bar.base(), WINDOW as u64);
    let b = bar.clone();
    start_log(move || {
        let regs = (b.wrote32(STS) as u64) << 32 | b.wrote32(CMD) as u64;
        (resets.load(Ordering::SeqCst) as u64) << 48 | regs
    });
    (hc, crate::setup::run(dev))
}

/// The routing writes in the log, in order, each with what the probe saw.
pub fn routing(log: &[Logged]) -> Vec<(Call, u64)> {
    log.iter().filter(|l| matches!(l.call, Call::PciConfig(..))).map(|l| (l.call, l.seen)).collect()
}

/// Where in the log `call` first is.
pub fn at(log: &[Logged], call: Call) -> usize {
    log.iter().position(|l| l.call == call).unwrap_or_else(|| panic!("{call:?} not in {log:?}"))
}
