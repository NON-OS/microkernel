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

/*
 * 09  PROVING: the proof being built on this machine, with how far it has
 * got and how long it has taken, then its hand-off to the relayer. Every
 * error the prover can raise arrives with its own sentence.
 */

use alloc::format;
use nonos_app_skeleton::PaintBuffer;

use crate::wallet::etna::backdrop::Backdrop;
use crate::wallet::etna::frame::{begin, end};
use crate::wallet::etna::frame_spec::FrameSpec;
use crate::wallet::etna::parts::fact::fact;
use crate::wallet::etna::tokens::{BAD, CYAN, GAP, LINE_2, TEXT_3};
use crate::wallet::etna::wrap::wrapped;
use crate::wallet::etna::Role;
use crate::wallet::screen::hits;
use crate::wallet::state::State;

const IDLE: &str = "Nothing is being proved right now. A proof starts when you confirm a \
     private payment or a withdrawal, and can take several minutes on one core.";

pub fn proving(state: &State, fb: &mut PaintBuffer) {
    hits::clear();
    let status = super::status::parts(state);
    let spec = FrameSpec {
        number: "09",
        title: "Proving",
        back: true,
        backdrop: Some(Backdrop::Proving),
        failure: super::absent::banner(state),
        footer: &[],
        status: &status,
        scroll: state.scroll,
    };
    let mut l = begin(fb, &spec);
    let c = l.content;
    let (x, w) = (c.x as i32, c.w as i32);
    let mut y = c.y;
    match &state.shield_ui.job {
        None => y += wrapped(fb, x, y as i32, w, Role::Lead, IDLE, TEXT_3) as u32,
        Some(job) => {
            y += fact(fb, c.x, y, c.w, "for", super::history_labels::kind(job.kind));
            let (m, s) = (job.elapsed_s / 60, job.elapsed_s % 60);
            y += fact(fb, c.x, y, c.w, "elapsed", &format!("{m} min {s:02} s"));
            y += fact(fb, c.x, y, c.w, "progress", &format!("{} of {}", job.done, job.of));
            fb.fill_round(c.x, y + GAP / 2, c.w, 6, 3, LINE_2);
            let lit = c.w * job.done.min(job.of) as u32 / job.of.max(1) as u32;
            fb.fill_round(c.x, y + GAP / 2, lit.max(1), 6, 3, CYAN);
            y += GAP + 6;
            if let Some(err) = job.error {
                y += wrapped(fb, x, y as i32, w, Role::Lead, err, BAD) as u32;
            }
        }
    }
    hits::reach(y, state.scroll, l.content_bottom);
    end(fb, &spec, &mut l);
    super::edges::put(state, &l);
}
