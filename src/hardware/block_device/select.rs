// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! Which disk the block layer reads and writes: the one NONOS lives on.
//!
//! A machine may carry the firmware's boot disk on AHCI next to the data disk
//! on NVMe, or an installer's blank target next to the disk it boots from. A
//! disk that merely answers is not the one to write, so each backend is asked
//! in turn whether its disk carries the store header or the disk plan, and the
//! first that does is kept for the boot. Nothing is kept before that: a driver
//! still starting, or a caller without the rights to ask, is asked again.

use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use spin::Once;

use super::announce::announce;
use super::backend::Backend;
use super::identify::{identify, Found};
use super::seen::{line, Seen};
use super::BlockDeviceError;

static SELECTED: Once<Backend> = Once::new();
static TOLD_NONE: AtomicBool = AtomicBool::new(false);

/// Asked in this order. A USB stick that carries NONOS is first: it is
/// there because someone brought it to boot a live session from it, and
/// that session keeps its state on the stick, not on an internal disk that
/// may hold an installed NONOS. The kernel is not told which disk the
/// firmware booted, so this order stands in for that answer. Then NVMe
/// before SATA, so a machine with an NVMe data disk and a SATA boot disk
/// settles on the NVMe one without reading the other; virtio-blk last.
/// The cost: on a machine with an xHCI controller, every disk waits while
/// driver.usb_msc0 looks for a device, about 1.5 s when none is plugged in.
const ORDER: [Backend; 4] = [Backend::UsbMsc, Backend::Nvme, Backend::Ahci, Backend::VirtioBlk];

/// The disk already kept for this boot, without asking any disk.
pub fn chosen() -> Option<Backend> {
    SELECTED.get().copied()
}

pub fn selected() -> Result<Backend, BlockDeviceError> {
    if let Some(&backend) = SELECTED.get() {
        return Ok(backend);
    }
    for backend in ORDER {
        let (found, seen) = identify(backend);
        say(backend, &seen);
        match found {
            Found::Layout => return Ok(*SELECTED.call_once(|| announce(backend))),
            Found::Absent => {}
            /*
             * A backend that could not be asked is not passed over: the disk
             * behind it may be the right one, and settling on a later disk
             * now would split reads and writes across two disks.
             */
            Found::Refused(e) => return Err(e),
        }
    }
    if !TOLD_NONE.swap(true, Ordering::Relaxed) {
        let line = "[BLOCK] no disk carries the NONOS store or disk plan yet; block I/O refused";
        crate::sys::serial::println(line.as_bytes());
        crate::log::warn!("{}", line);
    }
    Err(BlockDeviceError::Dead)
}

/// The last line said for each backend, and for the USB driver's own
/// search, by its hash: a disk asked on every store read is said only when
/// what it shows changes.
static SAID: [AtomicU64; 5] = [const { AtomicU64::new(0) }; 5];
const USB_SEARCH: usize = 4;

fn say(backend: Backend, seen: &Seen) {
    once(backend as usize, &line(backend, seen));
    /* The stick driver cannot write to the log; its search is said here. */
    if backend == Backend::UsbMsc {
        if let Some(report) = crate::hardware::usb_msc_capsule::report_line() {
            once(USB_SEARCH, &report);
        }
    }
}

fn once(slot: usize, text: &str) {
    let hash = text
        .bytes()
        .fold(0xcbf2_9ce4_8422_2325u64, |h, b| (h ^ u64::from(b)).wrapping_mul(0x100_0000_01b3));
    if SAID[slot].swap(hash, Ordering::Relaxed) != hash {
        crate::sys::serial::println(text.as_bytes());
    }
}
