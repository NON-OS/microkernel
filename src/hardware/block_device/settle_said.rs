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

//! The line the end of a pass that found no boot disk is said with, once:
//! what the loader recorded and what became of the other NONOS disk, so
//! `log BLOCK` shows why a boot came up as the disk it came up as.

use alloc::format;
use core::sync::atomic::{AtomicBool, Ordering};

use super::backend::Backend;
use super::other_disk::Other;

static TOLD: AtomicBool = AtomicBool::new(false);

/// `seen`: the loader named its partition, came through USB, and read its
/// store copy from that disk. Nothing is said while a disk is waited for.
pub(super) fn said(end: Other, seen: [bool; 3], other: Backend) {
    if end == Other::Wait || TOLD.swap(true, Ordering::Relaxed) {
        return;
    }
    let [named, usb, own] = seen.map(|b| if b { "yes" } else { "no" });
    let what = match end {
        Other::Keep => "is kept",
        _ => {
            "is not kept; this boot keeps nothing and reads the loader's copy of the store, if any"
        }
    };
    let line = format!(
        "[BLOCK] no disk driven holds the loader's partition (named {named}, through USB {usb}, \
         its store copy from that disk {own}): {} {what}",
        other.shown()
    );
    crate::sys::serial::println(line.as_bytes());
    crate::log::warn!("{}", line);
}
