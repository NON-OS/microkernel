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

//! The one request a stopped write ended on, in the driver's words when a
//! driver refused it and in the writer's otherwise, with the disk named:
//! on a machine with no serial port the failed screen is the whole report.

use alloc::format;
use alloc::string::String;

use nonos_blk_client::{describe_status, Refusal, RefusedOp};

use crate::install::format::bytes;
use crate::install::state::State;

/// The writer's own account, when no driver request was recorded failing:
/// "NVMe  ...  256 GB: write of 64 sectors at LBA 160256, in the ESP's data
/// area, failed: I/O error."
pub(super) fn attempt_text(state: &State, request: &str, status: Option<i32>) -> String {
    let why = status.map(describe_status).unwrap_or_else(|| String::from("see above"));
    format!("{}: {request}, failed: {why}.", disk_text(state).as_deref().unwrap_or("the disk"))
}

fn disk_text(state: &State) -> Option<String> {
    state.selected_disk().map(|d| {
        let model = d.identity.map(|i| String::from(i.model_str())).unwrap_or_default();
        let mut s = String::from(d.label());
        if !model.is_empty() {
            s.push_str("  ");
            s.push_str(&model);
        }
        if d.bytes() > 0 {
            s.push_str("  ");
            s.push_str(&bytes(d.bytes()));
        }
        s
    })
}

/// "NVMe  Samsung SSD 980  256 GB: write of 64 sectors at LBA 2080 failed:
/// NVMe status type 0 code 0x04, data transfer error."
pub(super) fn request_text(state: &State, r: Refusal) -> String {
    let disk = disk_text(state);
    let disk = disk.as_deref().unwrap_or("the disk");
    let why = describe_status(r.status);
    match r.op {
        RefusedOp::Flush => format!("{disk}: the cache flush failed: {why}."),
        op => format!(
            "{disk}: {} of {} sectors at LBA {} failed: {why}.",
            op.word(),
            r.sectors,
            r.lba
        ),
    }
}
