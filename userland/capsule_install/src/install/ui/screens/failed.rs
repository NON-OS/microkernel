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

//! Why it stopped, in the words the writer gave, what state the disk is in,
//! and what to do. A write that stopped before the table was complete left
//! a disk that must not be booted; a read-back that failed left a complete
//! disk whose contents cannot be trusted. Both are said.

use alloc::format;
use alloc::string::String;

use nonos_app_skeleton::PaintBuffer;
use nonos_blk_client::{describe_status, last_refusal, Refusal, RefusedOp};

use crate::install::format::bytes;
use crate::install::state::State;
use crate::install::ui::frame::Body;
use crate::install::ui::theme;
use crate::install::ui::widgets::section;
use crate::install::ui::wrap::Ink;

use super::failed_text::{RETRY_OTHER, RETRY_READBACK, RETRY_TABLE};

pub fn paint(state: &State, fb: &mut PaintBuffer, b: Body) {
    let why = state
        .outcome
        .as_ref()
        .and_then(|o| o.error.as_deref())
        .or(state.notice.as_deref())
        .unwrap_or("stopped for a reason the writer did not name");
    let m = &b.m;
    let red = (theme::DANGER, theme::DANGER);
    let fg = Ink::body(m, theme::FOREGROUND);
    let mut y = section(fb, m, b.x, b.y, b.w, ("01", "WHAT HAPPENED", red.0), why, fg);
    let Some(o) = state.outcome.as_ref() else {
        section(fb, m, b.x, y, b.w, ("02", "WHAT TO DO", red.1), RETRY_OTHER, fg);
        return;
    };
    /* A receipt exists only once the table and the flush went down. */
    let table_written = o.disk_guid[0] != b'-';
    let (disk, retry) = if table_written {
        ("It has a complete table, but its contents did not read back as written. Do not boot it.", RETRY_READBACK)
    } else {
        ("The write stopped before the partition table was complete. Do not boot it.", RETRY_TABLE)
    };
    let now = format!("{disk} {} were written before it stopped.", bytes(o.bytes_written));
    let muted = Ink::body(m, theme::MUTED);
    y = section(fb, m, b.x, y, b.w, ("02", "THE DISK NOW", red.0), &now, muted);
    /* The disk and the one request it refused, as the driver answered it:
     * on a machine with no serial port this screen is the whole report. */
    let refused = last_refusal()
        .map(|r| request_text(state, r))
        .or_else(|| o.request.as_deref().map(|req| attempt_text(state, req, o.status)));
    let step = if let Some(text) = refused.as_deref() {
        y = section(fb, m, b.x, y, b.w, ("03", "THE REQUEST", red.0), text, muted);
        "04"
    } else {
        "03"
    };
    section(fb, m, b.x, y, b.w, (step, "WHAT TO DO", red.1), retry, fg);
}

/// The writer's own account, when no driver request was recorded failing:
/// "NVMe  ...  256 GB: write of 64 sectors at LBA 160256, in the ESP's data
/// area, failed: I/O error."
fn attempt_text(state: &State, request: &str, status: Option<i32>) -> String {
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
fn request_text(state: &State, r: Refusal) -> String {
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
