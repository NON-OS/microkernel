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

//! A keyboard taken to report mode: the map read whole with Read Blob,
//! each Report's Reference and CCCD found, Protocol Mode written to
//! report mode, and the reports decoded by the map.

use super::maps::KEYBOARD;
use crate::sim::report_devices::*;
use crate::sim::Run;
use nonos_bt_hogp::host::Kind;
use nonos_bt_hogp::report::Input::*;

#[test]
fn the_map_is_read_with_read_blobs_from_each_length() {
    let mut run = Run::new(report_keyboard(&KEYBOARD));
    ready(&mut run);
    let reads: Vec<_> = run.sent.iter().filter(|p| p[0] == 0x0a || p[0] == 0x0c).collect();
    assert_eq!(reads[0], &[0x0a, MAP as u8, 0]);
    let offsets: Vec<u16> = reads[1..5].iter().map(|p| u16::from_le_bytes([p[3], p[4]])).collect();
    assert_eq!(offsets, [22, 44, 66, 88]);
    assert!(reads[1..5].iter().all(|p| p[0] == 0x0c && p[1] == MAP as u8));
    let finds = run.sent.iter().filter(|p| p[0] == 0x04).count();
    assert_eq!(finds, 3, "one per Report; their CCCDs are not looked for again");
}

#[test]
fn report_mode_is_set_and_the_reports_turned_on() {
    let mut run = Run::new(report_keyboard(&KEYBOARD));
    ready(&mut run);
    let s = run.host.sources();
    let got: Vec<_> = s.iter().map(|s| (s.kind, s.value, s.cccd)).collect();
    let want = [(Kind::Report, KEYS_IN, Some(KEYS_IN + 2)), (Kind::Report, CONSUMER_IN, Some(17))];
    assert_eq!(got, want, "the Reports take the boot keyboard's place");
    let w = &run.peer.writes;
    assert_eq!(w[0], (MODE, vec![1], true), "report mode");
    assert_eq!(w[1], (KEYS_IN + 2, vec![1, 0], false));
    assert_eq!(w[2], (CONSUMER_IN + 1, vec![1, 0], false));
    assert_eq!(w.len(), 3, "the boot keyboard's CCCD is left off");
}

#[test]
fn reports_become_keys_and_caps_lock_lights_the_report_led() {
    let mut run = Run::new(report_keyboard(&KEYBOARD));
    ready(&mut run);
    assert_eq!(
        run.notify(KEYS_IN, &[0x02, 0, 0x04, 0, 0, 0, 0, 0]),
        [KeyDown(0xe1), KeyDown(0x04)]
    );
    assert_eq!(run.notify(CONSUMER_IN, &[0xe9, 0]), [KeyDown(0x80)]);
    assert_eq!(run.notify(BOOT_IN, &[0, 0, 0x05, 0, 0, 0, 0, 0]), [], "the boot report is off");
    assert_eq!(run.notify(KEYS_IN, &[0, 0, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01]), []);
    let before = run.peer.writes.len();
    assert_eq!(
        run.notify(KEYS_IN, &[0, 0, 0x39, 0, 0, 0, 0, 0]),
        [KeyUp(0xe1), KeyUp(0x04), KeyDown(0x39)]
    );
    run.go();
    assert_eq!(run.peer.writes[before..], [(LEDS_OUT, vec![0x02], true)], "not the boot output");
    assert_eq!(run.host.disconnected(), [KeyUp(0x39), KeyUp(0x80)]);
}
