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
//! Whether a GOP handle is the console's, and which handle was latched, for
//! the line `[GOP] handle` on the boot log.

use crate::log::logger::log_warn;
use alloc::format;
use alloc::string::String;
use core::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
use uefi::prelude::*;
use uefi::proto::unsafe_protocol;
use uefi::table::boot::{BootServices, OpenProtocolParams};

/// EFI_CONSOLE_OUT_DEVICE_GUID (UEFI 2.10 section 12.1), a tag protocol
/// with no interface, on each handle that is part of the console output.
#[unsafe_protocol("d3b36f2c-d551-11d4-9a46-0090273fc14d")]
struct ConsoleOutDevice;

static HANDLES: AtomicUsize = AtomicUsize::new(0);
static CHOSEN: AtomicUsize = AtomicUsize::new(0);
/// Bit i set when handle i carries ConsoleOut; the first 32 are kept.
static CONSOLE_MASK: AtomicU32 = AtomicU32::new(0);

pub(super) fn is_console(bs: &BootServices, h: Handle) -> bool {
    let params = OpenProtocolParams { handle: h, agent: bs.image_handle(), controller: None };
    bs.test_protocol::<ConsoleOutDevice>(params).is_ok()
}

pub(super) fn note_chosen(console: &[bool], index: usize) {
    let mask = console.iter().take(32).enumerate().fold(0u32, |m, (i, &c)| m | ((c as u32) << i));
    HANDLES.store(console.len(), Ordering::SeqCst);
    CHOSEN.store(index, Ordering::SeqCst);
    CONSOLE_MASK.store(mask, Ordering::SeqCst);
}

/// One line naming the GOP latched, so a photo of a two-GPU laptop's log
/// says which one the splash and the kernel scan out through.
pub fn report_gop_handle() {
    let n = HANDLES.load(Ordering::Relaxed);
    if n == 0 {
        return;
    }
    let mask = CONSOLE_MASK.load(Ordering::Relaxed);
    let mut on = String::new();
    for i in (0..n.min(32)).filter(|&i| mask & (1 << i) != 0) {
        on.push_str(&format!(" {}", i + 1));
    }
    let i = CHOSEN.load(Ordering::Relaxed) + 1;
    let on = if on.is_empty() { String::from(" none") } else { on };
    log_warn("gop", &format!("[GOP] handle {} of {}, console out on:{}", i, n, on));
}
