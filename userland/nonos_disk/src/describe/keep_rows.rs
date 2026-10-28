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

//! What an update keeps and writes, in the rows an install shows for the
//! store and the boot partition, with what it leaves in place of what an
//! install erases. Read off the plan, so the screen says what the writer
//! will do.

use alloc::format;
use alloc::vec::Vec;

use super::rows::Row;
use super::shared::{boot_text, store_rows};
use super::size::size_text;
use crate::carry::Carried;
use crate::session::Plan;

pub fn describe_keep(plan: &Plan<'_>, carried: &Carried) -> Vec<Row> {
    let data = &plan.layout.data;
    let mut rows = Vec::new();
    let mut row = |label, value| rows.push(Row { label, value });
    let volume = size_text(data.bytes());
    row("kept", format!("data volume ({volume}), its key, the plan and the table"));
    store_rows(plan, carried, &mut row);
    row("boot", boot_text(plan));
    row("not kept", "store apps, settings since setup, the wallet's vault, Wi-Fi".into());
    rows
}
