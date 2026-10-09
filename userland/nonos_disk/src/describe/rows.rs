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

//! The rows, in the order the disk is laid out.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use nonos_disk_map::MAX_TOTAL_BYTES;

use super::carried::carried_text;
use super::size::size_text;
use crate::carry::Carried;
use crate::session::Plan;

pub struct Row {
    pub label: &'static str,
    pub value: String,
}

pub fn describe(plan: &Plan<'_>, carried: &Carried) -> Vec<Row> {
    let (l, store, image) = (&plan.layout, &plan.store, &plan.image);
    let disk = size_text(l.total_sectors.saturating_mul(crate::sink::SECTOR_SIZE as u64));
    let mut rows = Vec::new();
    let mut row = |label, value| rows.push(Row { label, value });
    row("erased", format!("all {disk} of it, whatever it holds now"));
    row("table", String::from("GPT, 4 partitions, backup copy at the end"));
    let (files, payload) = (store.files, size_text(store.payload_bytes));
    row(
        "store",
        format!(
            "{} at LBA {}: {files} files, {payload}",
            size_text(l.store.bytes()),
            l.store.first
        ),
    );
    row("carried", carried_text(carried));
    if carried.left_out > 0 {
        let (n, bytes) = (carried.left_out, size_text(carried.left_out_bytes));
        let (s, most) = (if n == 1 { "" } else { "s" }, size_text(MAX_TOTAL_BYTES));
        row("left out", format!("{n} program{s}, {bytes}: more than the store's {most}"));
    }
    if carried.skipped > 0 {
        let (n, s) = (carried.skipped, if carried.skipped == 1 { "" } else { "s" });
        row("not carried", format!("{n} signed program{s} that could not be carried whole"));
    }
    row("disk plan", format!("LBA {}, no imports; key header cleared", l.plan.first));
    row(
        "data volume",
        format!("{} at LBA {}, formatted on first boot", size_text(l.data.bytes()), l.data.first),
    );
    let (loader, kernel) =
        (size_text(image.boot_efi.len() as u64), size_text(image.kernel_bin.len() as u64));
    row(
        "boot",
        format!("{} at the end: loader {loader}, kernel {kernel}", size_text(l.esp.bytes())),
    );
    rows
}
