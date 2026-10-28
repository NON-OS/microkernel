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

//! The rows an install and an update both show: the store, what it
//! carries and what it could not, and the boot partition.

use alloc::format;
use alloc::string::String;

use nonos_disk_map::MAX_TOTAL_BYTES;

use super::carried::carried_text;
use super::size::size_text;
use crate::carry::Carried;
use crate::session::Plan;

pub(super) fn store_rows(plan: &Plan<'_>, c: &Carried, row: &mut impl FnMut(&'static str, String)) {
    let (l, store) = (&plan.layout, &plan.store);
    let (files, payload) = (store.files, size_text(store.payload_bytes));
    let size = size_text(l.store.bytes());
    row("store", format!("{size} at LBA {}: {files} files, {payload}", l.store.first));
    row("carried", carried_text(c));
    if c.left_out > 0 {
        let (n, bytes) = (c.left_out, size_text(c.left_out_bytes));
        let (s, most) = (if n == 1 { "" } else { "s" }, size_text(MAX_TOTAL_BYTES));
        row("left out", format!("{n} program{s}, {bytes}: more than the store's {most}"));
    }
    if c.skipped > 0 {
        let (n, s) = (c.skipped, if c.skipped == 1 { "" } else { "s" });
        row("not carried", format!("{n} signed program{s} that could not be carried whole"));
    }
}

pub(super) fn boot_text(plan: &Plan<'_>) -> String {
    let (l, image) = (&plan.layout, &plan.image);
    let (loader, kernel) =
        (size_text(image.boot_efi.len() as u64), size_text(image.kernel_bin.len() as u64));
    format!("{} at the end: loader {loader}, kernel {kernel}", size_text(l.esp.bytes()))
}
