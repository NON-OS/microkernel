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

//! Saying what each disk showed while the block layer chose, once per
//! change.

use core::sync::atomic::{AtomicU64, Ordering};

use super::backend::Backend;

/// The last line said for each backend, and for the USB driver's own
/// search, by its hash: a disk asked on every store read is said only when
/// what it shows changes. A block-wire disk keeps its own in its slot.
static SAID: [AtomicU64; 5] = [const { AtomicU64::new(0) }; 5];
const USB_SEARCH: usize = 4;

fn slot(backend: Backend) -> &'static AtomicU64 {
    match backend {
        Backend::VirtioBlk => &SAID[0],
        Backend::Ahci => &SAID[1],
        Backend::Nvme => &SAID[2],
        Backend::UsbMsc => &SAID[3],
        Backend::Wire(i) => super::wire::said(i),
    }
}

pub(super) fn say(backend: Backend, line: &str) {
    once(slot(backend), line);
    /* The stick driver cannot write to the log; its search is said here. */
    if backend == Backend::UsbMsc {
        if let Some(report) = crate::hardware::usb_msc_capsule::report_line() {
            once(&SAID[USB_SEARCH], &report);
        }
    }
}

fn once(slot: &AtomicU64, text: &str) {
    let hash = text
        .bytes()
        .fold(0xcbf2_9ce4_8422_2325u64, |h, b| (h ^ u64::from(b)).wrapping_mul(0x100_0000_01b3));
    if slot.swap(hash, Ordering::Relaxed) != hash {
        crate::sys::serial::println(text.as_bytes());
    }
}
