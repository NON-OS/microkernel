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

//! A step of the background scan between requests, or the open port serviced.

use crate::firmware::gen3::heard::hear;
use crate::firmware::gen3::rx_frame::RxFrame;
use crate::firmware::gen3::sweep::Tick;

use super::clock::{now_ms, Uptime};
use super::say::Line;
use super::state::{Radio, State, REST_MS};

impl Radio {
    /// Advance the background scan by one step, or service the open port.
    pub fn tick(&mut self) {
        if let State::Restarting(_) = self.state {
            self.restart_due(now_ms());
            return;
        }
        if self.joined() {
            self.service_link();
            return;
        }
        let State::Up(up) = &mut self.state else { return };
        let now = now_ms();
        let results = &mut self.results;
        let beacons = &mut self.beacons;
        let mut on_frame = |f: RxFrame| {
            if hear(results, &f) {
                *beacons = beacons.saturating_add(1);
            }
        };
        let tick = if !self.sweep.running() && now >= self.next_ms {
            match self.sweep.start(&mut up.dev, &mut Uptime, &up.channels, now, &mut on_frame) {
                Ok(t) => t,
                Err(_) if up.dev.error_cause() => Tick::Failed,
                Err(_) => {
                    Line::new().text(b"scan request not answered; retrying after a rest").send();
                    self.next_ms = now.saturating_add(REST_MS);
                    Tick::Idle
                }
            }
        } else {
            self.sweep.pump(&mut up.dev, now, &mut on_frame)
        };
        self.after_step(tick, now);
    }
}
