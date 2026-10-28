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

use super::shared::{boot_text, store_rows};
use super::size::size_text;
use crate::carry::Carried;
use crate::session::Plan;

pub struct Row {
    pub label: &'static str,
    pub value: String,
}

pub fn describe(plan: &Plan<'_>, carried: &Carried) -> Vec<Row> {
    let l = &plan.layout;
    let disk = size_text(l.total_sectors.saturating_mul(crate::sink::SECTOR_SIZE as u64));
    let mut rows = Vec::new();
    let mut row = |label, value| rows.push(Row { label, value });
    row("erased", format!("all {disk} of it, whatever it holds now"));
    row("table", String::from("GPT, 4 partitions, backup copy at the end"));
    store_rows(plan, carried, &mut row);
    row("disk plan", format!("LBA {}, no imports; key header cleared", l.plan.first));
    row(
        "data volume",
        format!("{} at LBA {}, formatted on first boot", size_text(l.data.bytes()), l.data.first),
    );
    row("boot", boot_text(plan));
    rows
}
