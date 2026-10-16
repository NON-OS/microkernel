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

//! F-108: the critical watches keep their clocks while the ACPI capsule is
//! down. A seeded run of reports, deaths, restarts, a restart limit and
//! the clock, against a model: a zone last reported at or past _HOT or
//! _CRT, or a battery last reported critical, is powered off once its
//! grace has run, reports or none; a reading without the condition clears
//! it; a death or a restart changes nothing; one never reported, never.

use super::critical_watch::{Action, Watch};
use super::platform_event::{CRITICAL, HOT, PASSIVE};
use super::thermal_watch::alarming;

#[path = "clock_model.rs"]
mod clock_model;
use clock_model::{late, model, roll};

#[test]
fn the_grace_runs_on_the_clock_through_deaths_and_restarts() {
    let s = &mut 0x9E37_79B9_7F4A_7C15u64;
    let (mut offs_down, mut offs_spent, mut cleared, mut boots) = (0, 0, 0, 0);
    while boots < 400 {
        boots += 1;
        let (zone, battery) = (Watch::new(), Watch::new());
        let (mut za, mut ba, mut now, mut up, mut gave_up, mut restarted) =
            (None, None, 1_000u64, true, false, false);
        for _ in 0..2_000 {
            now += roll(s, 8_000);
            match roll(s, 10) {
                0 | 1 if up => {
                    let word =
                        [3000, PASSIVE | 3500, HOT | 3700, CRITICAL | 3800][roll(s, 4) as usize];
                    let bad = alarming(word);
                    cleared += usize::from(restarted && !bad && za.is_some());
                    assert_eq!(zone.step(bad, now), model(&mut za, bad, now));
                }
                2 if up => {
                    let bad = roll(s, 3) == 0;
                    assert_eq!(battery.step(bad, now), model(&mut ba, bad, now));
                }
                3 if up => up = false,
                4 if !up && !gave_up => (up, restarted) = (true, true),
                5 if !up => gave_up = roll(s, 4) == 0,
                _ => {}
            }
            let (z, b) = (zone.tick(now), battery.tick(now));
            assert_eq!((z, b), (late(za, now), late(ba, now)), "boot {boots} at {now}");
            if z == Action::PowerOff || b == Action::PowerOff {
                offs_down += usize::from(!up);
                offs_spent += usize::from(gave_up);
                break;
            }
        }
    }
    let says = format!("{offs_down} off while down, {offs_spent} after the limit, {cleared}");
    assert!(offs_down >= 50 && offs_spent >= 10 && cleared >= 20, "{says}");
}
