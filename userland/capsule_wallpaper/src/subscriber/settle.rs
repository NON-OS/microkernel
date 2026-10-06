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

use nonos_libc::Look;

use super::job::worker::{poll, Answer};
use super::job::Outcome;
use super::say;
use crate::paint::paint_image;
use crate::server::scene::request_commit;
use crate::state::Context;

/// Take a job's answer when it is in, and show the picture it brought when
/// it is still the one wanted.
pub fn settle(ctx: &mut Context) {
    let Some(running) = ctx.plan.running() else {
        return;
    };
    let Answer { index, outcome } = match poll() {
        Look::Waiting => return,
        Look::Ready(answer) => answer,
        // Not reached: a job out leaves the slot Running until its answer is
        // in. Should it be, the job is over with nothing to show.
        Look::Idle => Answer { index: running, outcome: Outcome::Failed("fetching it") },
    };
    if ctx.plan.wanted() == Some(index) {
        match &outcome {
            Outcome::Decoded(_) => {}
            Outcome::Stopped(download) => {
                let (at, size) = download.progress();
                say::stopped(index, at, size);
            }
            Outcome::Failed(step) => say::failed(index, step),
        }
    }
    let Some(image) = ctx.plan.finish(index, outcome) else {
        return;
    };
    if !paint_image(ctx, &image) {
        say::failed(index, "painting it");
        ctx.plan.missed(index);
        return;
    }
    drop(image);
    request_commit(ctx);
    ctx.plan.shown(index);
    say::shown(index);
}
