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

//! The window: the memory floor and the request first, then Enter runs the
//! steps one a tick. Esc closes; an on-demand window exits on close, and the
//! kernel zeroizes everything it held.

use alloc::vec::Vec;

use nonos_app_skeleton::{App, AppManifest, EventOutcome, InputEvent, InputKind, PaintBuffer};

use super::memory::Memory;
use super::state::{Line, Stage, Work};
use super::{flow, manifest, paint};
use crate::assemble::wipe;

pub struct Prove {
    pub(super) memory: Memory,
    pub(super) stage: Stage,
    pub(super) log: Vec<Line>,
    pub(super) work: Work,
}

impl App for Prove {
    fn manifest(&self) -> AppManifest {
        manifest::manifest()
    }
    fn on_event(&mut self, event: InputEvent) -> EventOutcome {
        if event.kind != InputKind::KeyDown {
            return EventOutcome::Idle;
        }
        self.key(event.code)
    }
    fn paint(&mut self, fb: &mut PaintBuffer) {
        paint::paint(fb, &self.log, self.stage);
    }
    fn on_tick(&mut self) -> bool {
        let Stage::Running(step) = self.stage else {
            return false;
        };
        self.stage = flow::advance(step, &mut self.work, &mut self.log);
        true
    }
    fn busy(&self) -> bool {
        matches!(self.stage, Stage::Running(_))
    }
    /* Closed mid-run: the witness is cleared before the window goes. */
    fn close_requested(&mut self) -> bool {
        if let Some(mut witness) = self.work.witness.take() {
            wipe(&mut witness);
        }
        true
    }
}
