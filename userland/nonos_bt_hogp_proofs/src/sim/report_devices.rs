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

//! A keyboard that offers report mode (HIDS 1.0, 3): Protocol Mode, a
//! Report Map, the boot reports, and three Reports, each with a Report
//! Reference, in the order HIDS 1.0 leaves to the device.

use super::db::Db;
use super::devices::{NOTIFY, READ, WRITE, WWR};
use super::run::Run;
use nonos_bt_hogp::host::Outcome;

pub const MODE: u16 = 3;
pub const MAP: u16 = 5;
pub const BOOT_IN: u16 = 7;
pub const BOOT_OUT: u16 = 10;
/// Report 1, input: its value, then its Reference, then its CCCD.
pub const KEYS_IN: u16 = 12;
/// Report 2, input: its value, then its CCCD, then its Reference.
pub const CONSUMER_IN: u16 = 16;
/// Report 1, output, for the LEDs.
pub const LEDS_OUT: u16 = 20;

/// The keyboard with `map` as its Report Map, and its Report References
/// as given: report 1 input, report 2 input, report 1 output.
pub fn report_keyboard_with(map: &[u8], refs: [&[u8]; 3]) -> Db {
    Db::default()
        .service(0x1812)
        .char(READ | WWR, 0x2a4e, &[1])
        .char(READ, 0x2a4b, map)
        .char(READ | NOTIFY, 0x2a22, &[0; 8])
        .desc(0x2902, &[0, 0])
        .char(READ | WRITE | WWR, 0x2a32, &[0])
        .char(READ | NOTIFY, 0x2a4d, &[0; 8])
        .desc(0x2908, refs[0])
        .desc(0x2902, &[0, 0])
        .char(READ | NOTIFY, 0x2a4d, &[0; 2])
        .desc(0x2902, &[0, 0])
        .desc(0x2908, refs[1])
        .char(READ | WRITE | WWR, 0x2a4d, &[0])
        .desc(0x2908, refs[2])
}

pub fn report_keyboard(map: &[u8]) -> Db {
    report_keyboard_with(map, [&[1, 1], &[2, 1], &[1, 2]])
}

/// Discovery run to its end, with a keyboard and no mouse found.
pub fn ready(run: &mut Run) {
    assert_eq!(run.go(), Some(Outcome::Ready { keyboard: true, mouse: false }));
}
