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

//! The takeover model's invariants (L79), each a bit: those every step is
//! held to, those the step that acts checks, and those only a world where
//! nothing can move is held to.

use super::types::Stage;
use super::world::{code, END};
use super::world_state::World;

pub(super) const TWO_WRITERS: u16 = 1;
pub(super) const KERNEL_DRAWS: u16 = 1 << 1;
pub(super) const FB_AFTER: u16 = 1 << 2;
pub(super) const DARK: u16 = 1 << 3;
pub(super) const OUTLIVED: u16 = 1 << 4;
pub(super) const MARKED: u16 = 1 << 5;
pub(super) const CLOBBER: u16 = 1 << 6;
pub(super) const WIPE_LEFT: u16 = 1 << 7;
pub(super) const SPURIOUS: u16 = 1 << 8;
pub(super) const PARKED: u16 = 1 << 9;
pub(super) const STRANDED: u16 = 1 << 10;
pub(super) const TAKING: u16 = 1 << 11;
pub(super) const SPIN: u16 = 1 << 12;
pub(super) const STOPPED_LEFT: u16 = 1 << 13;

impl World {
    pub(super) fn fail(&mut self, bit: u16) {
        self.bad |= bit;
    }
    /// After every step.
    pub(super) fn check(&mut self) {
        let ready = self.ready();
        let fb_ready = self.fb.is_some_and(|e| e.stage == code(Stage::Ready));
        let outlived = (0..2).any(|r| {
            let shows = !self.dead && self.page[r].shows_fb();
            shows && self.fb.map(|e| e.epoch) != Some(r as u8 + 1)
        });
        let broken = [
            (ready && self.shown(), TWO_WRITERS),
            (ready && self.drawing(), KERNEL_DRAWS),
            (self.gone && fb_ready, FB_AFTER),
            (self.gone && !self.ever_in, DARK),
            (outlived, OUTLIVED),
        ];
        broken.iter().filter(|b| b.0).for_each(|b| self.fail(b.1));
    }
    /// Where nothing can move: every path ended, no entry left but a covering
    /// claim made ready, no takeover counted, no stop latched for nothing, and
    /// nothing left stopping the console unless a covering claim is ready.
    pub(super) fn end_broken(&self) -> u16 {
        let parked = self.pc.iter().any(|&p| p < END);
        let left = self.gpu.iter().flatten().any(|&s| s != code(Stage::Ready));
        let dark = self.gone && !self.ever_in;
        let ends = [(parked, PARKED), (left || self.fb.is_some(), STRANDED)];
        let ends = [ends[0], ends[1], (self.takers > 0, TAKING), (dark, DARK)];
        let stopped = !self.ready() && self.stopped();
        let ends = ends.into_iter().chain([(stopped, STOPPED_LEFT)]);
        ends.filter(|b| b.0).fold(0, |bits, b| bits | b.1)
    }
}
