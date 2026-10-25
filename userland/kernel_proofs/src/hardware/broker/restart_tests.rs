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

//! F-107: the ACPI capsule ending and restarting at any moment, against a
//! model of who should read the power button. A live capsule that took it
//! reads it; at every other moment the kernel's poll does, even while the
//! capsule's recorded pid still names the dead one; never both, never
//! neither. The poll's press is the power-off once any capsule served the
//! button, and a capsule that takes it again hears it is a restart. A
//! Global Lock the capsule said it held is released at its exit, once.

use super::acpi::button_owner::{give_back, kernel_press, stands_down, take, Press};
use super::acpi::leave_rule::{lock_at_exit, press_in_gap, take_gap_press};

#[path = "restart_model.rs"]
mod model;
use model::{lock, Model};

#[test]
fn every_restart_gives_back_and_takes_again() {
    let _ = take(0); // the statics are the process's: start from a known state
    let (mut m, mut pid, s) = (Model::default(), 1000u32, &mut 0x2545_F491_4F6C_DD1Du64);
    let (mut exits, mut gap, mut served) = (0, 0, 0);
    for _ in 0..20_000 {
        *s ^= *s << 13;
        *s ^= *s >> 7;
        *s ^= *s << 17;
        let live = m.alive != 0;
        match *s % 8 {
            0 if !live => (pid, m.alive, m.recorded) = (pid + 1, pid + 1, pid + 1),
            1 if live => {
                assert!(take(m.alive), "every take after the first is a restart");
                m.taken = m.alive;
            }
            2 if live => lock(&mut m, true),
            3 if live => lock(&mut m, false),
            4 if live => {
                assert_eq!(give_back(m.alive), m.taken == m.alive);
                assert_eq!(lock_at_exit(m.alive), m.lock == m.alive);
                assert!(!lock_at_exit(m.alive), "released once");
                (m.taken, m.lock) = (0, 0);
                (m.alive, exits) = (0, exits + 1);
            }
            5 => m.recorded = m.alive,
            _ if stands_down(m.recorded) => {
                assert!(live && m.taken == m.alive, "the capsule reads only while it lives");
                served += 1;
            }
            _ => {
                assert!(m.taken == 0 || m.taken != m.alive, "the kernel reads only without one");
                assert_eq!(kernel_press(), Press::PowerOff);
                press_in_gap();
                assert!(take_gap_press() && !take_gap_press(), "acted on once");
                gap += 1;
            }
        }
    }
    let _ = (give_back(m.alive), lock_at_exit(m.alive));
    assert!(exits >= 500 && gap >= 300 && served >= 300, "{exits} exits {gap} gap {served}");
}
