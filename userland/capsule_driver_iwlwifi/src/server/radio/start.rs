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

//! The radio brought up once, at start.

use crate::firmware::gen3::outcome::Failure;
use crate::setup::{command::command, give_back};
use nonos_libc::{mk_exit, EXIT_ABSENT};
use nonos_wifi_core::scan_list::ScanResults;

use crate::driver::Driver;
use crate::firmware::gen3::sweep::Sweep;

use super::bring::{bring_up, Seen};
use super::join::Join;
use super::say::Line;
use super::state::{Radio, State};

impl Radio {
    /// Bring the radio up on the claimed card, or record why not.
    pub fn start(d: &mut Driver) -> Radio {
        let mut seen = Seen::default();
        let state = match bring_up(d, &mut seen) {
            Ok(up) => {
                Line::new().text(b"radio up; passive scanning").send();
                State::Up(up)
            }
            Err(f) => {
                // A card the bring-up touched is stopped; one it never touched
                // is left as setup left it, for the driver's own protocol.
                if seen.touched {
                    let _ = command(d.device_id, d.claim_epoch, false);
                }
                Line::new()
                    .text(b"radio not up: ")
                    .text(f.text().as_bytes())
                    .text(b" (step ")
                    .num(u32::from(f.step()))
                    .text(b", detail ")
                    .hex(f.detail())
                    .text(b")")
                    .send();
                // A card this driver cannot run is given back whole: none found.
                if matches!(f, Failure::Refused(_)) {
                    give_back(d);
                    mk_exit(EXIT_ABSENT);
                }
                State::Down(f)
            }
        };
        Radio {
            state,
            sweep: Sweep::new(),
            results: ScanResults::new(),
            beacons: 0,
            next_ms: 0,
            join: Join::new(),
            pci: (d.device_id, d.claim_epoch),
        }
    }
}
