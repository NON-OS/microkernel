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

//! Writing and reading back share one screen: a bar, the byte counts, the
//! rate. The rate is measured from the bytes the driver has acknowledged
//! over the time since the job started, so it is the disk's speed and not
//! a guess.

use nonos_app_skeleton::PaintBuffer;
use nonos_libc::mk_time_millis;

use crate::install::format::{bytes, percent, rate};
use crate::install::state::{Screen, State};
use crate::install::ui::frame::Body;
use crate::install::ui::metrics::{BODY_PX, LINE_H, TITLE_PX};
use crate::install::ui::text::right;
use crate::install::ui::widgets::bar;
use crate::install::ui::{text, theme};

pub fn paint(state: &State, fb: &mut PaintBuffer, b: Body) {
    let Some(job) = state.job.as_ref() else { return };
    let verifying = state.screen == Screen::Verifying;
    let what = if verifying {
        "Reading every sector back and comparing it with what was sent."
    } else {
        "Writing the bootloader and the kernel image to the disk."
    };
    text::line(fb, b.x, b.y, what, theme::FOREGROUND, BODY_PX);

    let y = b.y + 2 * LINE_H + 8;
    let pct = alloc::format!("{}%", percent(job.done, job.total));
    text::line(fb, b.x, y, &pct, theme::TITLE, TITLE_PX);
    let colour = if verifying { theme::OK } else { theme::ACCENT };
    bar(fb, b.x, y + 40, b.w, job.done, job.total, colour);

    let now = mk_time_millis().max(0) as u64;
    let elapsed = now.saturating_sub(job.started_ms);
    let counts = alloc::format!("{} of {}", bytes(job.done), bytes(job.total));
    text::line(fb, b.x, y + 70, &counts, theme::FOREGROUND, BODY_PX);
    let speed = if verifying {
        alloc::format!("{}  read", rate(job.done, elapsed.saturating_sub(job.write_seconds * 1000)))
    } else {
        alloc::format!("{}  write", rate(job.done, elapsed))
    };
    right(fb, b.x + b.w, y + 70, &speed, theme::MUTED, BODY_PX);

    if verifying {
        let wrote = alloc::format!("written in {} s, table and flush done", job.write_seconds);
        text::line(fb, b.x, y + 70 + LINE_H, &wrote, theme::MUTED, BODY_PX);
    }
}
