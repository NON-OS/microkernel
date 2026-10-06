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

//! The gen3 radio as the serving loop runs it. At start the card is brought
//! up once (`bring`); on an SO platform with bundled firmware that ends with
//! the firmware up and a passive scan ready to run. The loop then calls
//! `tick` between requests: a background sweep is started, pumped and, when
//! the firmware completes it, the scan list is aged and the next sweep waits
//! a rest. Beacons and probe responses go into the shared scan list as they
//! are received. A join (`join`) stops the sweep, hunts the network and
//! keys a link; while the port is open the background scan rests and `tick`
//! services the link instead.
//!
//! The broker DMA grants (`grant`), the uptime clock (`clock`), the console
//! lines (`say`) and the kernel's randomness (`address`, `entropy`) are the
//! capsule's side of the `Region` and `Clock` traits and the entropy the host
//! proofs supply directly.

mod address;
mod bring;
mod clock;
mod entropy;
mod grant;
mod join;
mod say;

use nonos_wifi_core::scan_list::ScanResults;

use crate::driver::Driver;
use crate::firmware::gen3::heard::hear;
use crate::firmware::gen3::outcome::{Failure, STAGE_READY};
use crate::firmware::gen3::rx_frame::RxFrame;
use crate::firmware::gen3::start::stop_device;
use crate::firmware::gen3::sweep::{Sweep, Tick};

use bring::{bring_up, Seen, Up};
use clock::{now_ms, Uptime};
use join::Join;
use say::Line;

/// The rest between one sweep's end and the next sweep's start.
const REST_MS: u64 = 3000;
/// Stalled sweeps reported on the console; later ones are only counted.
const STALLS_SAID: u32 = 3;

enum State {
    Up(Up),
    Down(Failure),
}

pub struct Radio {
    seen: Seen,
    state: State,
    sweep: Sweep,
    /// The networks heard, aged by sweep.
    pub results: ScanResults,
    /// Beacons and probe responses that parsed.
    pub beacons: u32,
    next_ms: u64,
    join: Join,
}

impl Radio {
    /// Bring the radio up on the claimed card, or record why not.
    pub fn start(d: &Driver) -> Radio {
        let mut seen = Seen::default();
        let state = match bring_up(d, &mut seen) {
            Ok(up) => {
                Line::new().text(b"radio up; passive scanning").send();
                State::Up(up)
            }
            Err(f) => {
                Line::new()
                    .text(b"radio not up: ")
                    .text(f.text().as_bytes())
                    .text(b" (step ")
                    .num(u32::from(f.step()))
                    .text(b", detail ")
                    .hex(f.detail())
                    .text(b")")
                    .send();
                State::Down(f)
            }
        };
        Radio {
            seen,
            state,
            sweep: Sweep::new(),
            results: ScanResults::new(),
            beacons: 0,
            next_ms: 0,
            join: Join::new(),
        }
    }

    /// The firmware is up and scanning.
    pub fn scanning(&self) -> bool {
        matches!(self.state, State::Up(_))
    }

    /// This path has touched the card, so nothing else may drive it.
    pub fn owns_card(&self) -> bool {
        self.seen.touched
    }

    /// The stage code, step and detail word for the status reply.
    pub fn stage(&self) -> (u8, u8, u32) {
        match &self.state {
            State::Up(_) => (STAGE_READY, 0, 0),
            State::Down(f) => (f.stage(), f.step(), f.detail()),
        }
    }

    pub fn hw_rev(&self) -> u32 {
        self.seen.hw_rev
    }

    pub fn rf_id(&self) -> u32 {
        self.seen.rf_id
    }

    /// Sweeps completed, sweeps stalled, and frames received.
    pub fn counts(&self) -> (u32, u32, u32) {
        (self.sweep.completed, self.sweep.stalled, self.sweep.frames)
    }

    /// Advance the background scan by one step, or service the open port.
    pub fn tick(&mut self) {
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
        match tick {
            Tick::Completed(_) => {
                if self.sweep.completed == 1 {
                    Line::new()
                        .text(b"first scan complete: ")
                        .num(self.results.count() as u32)
                        .text(b" networks from ")
                        .num(self.beacons)
                        .text(b" beacons")
                        .send();
                }
                self.results.end_sweep();
                self.next_ms = now.saturating_add(REST_MS);
            }
            Tick::Stalled => {
                if self.sweep.stalled <= STALLS_SAID {
                    Line::new().text(b"scan gave no completion within its budget").send();
                }
                self.results.end_sweep();
                self.next_ms = now.saturating_add(REST_MS);
            }
            Tick::Failed => {
                stop_device(up.dev.m, &mut Uptime);
                Line::new().text(b"radio down: ").text(Failure::Lost.text().as_bytes()).send();
                self.state = State::Down(Failure::Lost);
            }
            Tick::Idle | Tick::Running => {}
        }
    }
}
