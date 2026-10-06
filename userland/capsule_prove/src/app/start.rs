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

//! What the window shows before anything is asked of the TPM: whether the
//! prover's memory is held, and whom the request would prove to. Run again
//! from the start after a refusal or a saved proof.

use alloc::vec::Vec;

use nonos_app_skeleton::{EventOutcome, KEY_ENTER, KEY_ESC};

use super::memory::Memory;
use super::state::{Line, Mark, Stage, Step, Work};
use super::window::Prove;
use super::{paint, registry};

impl Prove {
    pub fn new(memory: Memory) -> Self {
        let mut p = Prove { memory, stage: Stage::Refused, log: Vec::new(), work: Work::default() };
        p.start();
        p
    }

    pub(super) fn start(&mut self) {
        self.log.clear();
        self.work = Work::default();
        let (held, text) = paint::memory_line(self.memory);
        self.log.push(Line { mark: if held { Mark::Ok } else { Mark::No }, text });
        self.stage = Stage::Refused;
        if !held {
            return;
        }
        match registry::request() {
            Ok(r) => {
                self.log.push(Line { mark: Mark::Note, text: registry::summary(&r) });
                self.work.request = Some(r);
                self.stage = Stage::Ready;
            }
            Err(text) => self.log.push(Line { mark: Mark::No, text }),
        }
    }

    /* Enter agrees to the request shown, or starts again; Esc closes. */
    pub(super) fn key(&mut self, code: u32) -> EventOutcome {
        match (code, self.stage) {
            (KEY_ESC, _) => EventOutcome::Close,
            (KEY_ENTER, Stage::Ready) => {
                self.stage = Stage::Running(Step::Registry);
                EventOutcome::Repaint
            }
            (KEY_ENTER, Stage::Refused | Stage::Done) => {
                self.start();
                EventOutcome::Repaint
            }
            _ => EventOutcome::Idle,
        }
    }
}
