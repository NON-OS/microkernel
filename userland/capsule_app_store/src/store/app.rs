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

use nonos_app_skeleton::{App, AppManifest, EventOutcome, InputEvent, PaintBuffer};

use super::event::on_event;
use super::fill::{Step, STEP_MS};
use super::manifest::manifest;
use super::state::State;
use super::ui::frame;

pub struct Store {
    state: State,
}

impl Store {
    pub fn new() -> Self {
        Store { state: State::new() }
    }
}

impl App for Store {
    fn manifest(&self) -> AppManifest {
        manifest()
    }
    fn on_event(&mut self, event: InputEvent) -> EventOutcome {
        on_event(&mut self.state, event)
    }
    fn paint(&mut self, fb: &mut PaintBuffer) {
        frame(&mut self.state, fb);
    }
    /// Asks the market what is still unasked, one listing a tick, and the
    /// system where a moving install stands. An idle store costs nothing.
    /// The catalogue itself is asked on the first tick, which the runner
    /// makes after the first frame is up: the window never waits on the
    /// market to appear.
    fn on_tick(&mut self) -> bool {
        /* A store opened before the market registered asks again, a tick
         * at a time, rather than holding "not running yet" until r. */
        if !self.state.loaded || self.state.market_not_up() {
            self.state.refresh();
            return true;
        }
        let moved = self.state.any_pending() && self.state.poll_pending();
        let fetched = self.state.poll_fetch();
        if moved {
            // "install requested" is stale once the system has said more.
            self.state.asked = None;
        }
        let filled = self.state.fill() == Step::Shown;
        moved || fetched || filled
    }
    fn tick_interval_ms(&self) -> i64 {
        match !self.state.loaded || self.state.filling() {
            true => STEP_MS,
            false => 500,
        }
    }
    /*
     * Never busy. A moving install is asked about on the 500 ms tick, which
     * the runner keeps by blocking on its inbox until the tick is due; busy
     * would cap that wait at a millisecond and spin the store for as long
     * as an install took. `busy` is left at the trait's default.
     */
}
