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

//! Keyboards that stay in boot mode: a map that does not parse, is too
//! long, cannot be read, or sits in a service with a boot mouse. Boot mode
//! is then exactly as before report mode existed.

use super::maps::KEYBOARD;
use crate::refuse_tests::failed;
use crate::sim::report_devices::*;
use crate::sim::{Peripheral, Run};
use nonos_bt_hogp::host::{Kind, Outcome};
use nonos_bt_hogp::report::map::MAP_MAX;
use nonos_bt_hogp::report::Input::*;
use nonos_bt_hogp::HogpError;
use nonos_bt_wire::att::error::{INSUFFICIENT_AUTHENTICATION, READ_NOT_PERMITTED};

/// Boot mode on the report keyboard: the boot report on, the Reports off,
/// and Caps Lock on the boot output report.
pub fn boot(run: &mut Run) {
    assert_eq!(run.go(), Some(Outcome::Ready { keyboard: true, mouse: false }));
    let s = run.host.sources();
    assert_eq!(
        s.iter().map(|s| (s.kind, s.value)).collect::<Vec<_>>(),
        [(Kind::Keyboard, BOOT_IN)]
    );
    assert_eq!(run.peer.writes, [(MODE, vec![0], true), (BOOT_IN + 1, vec![1, 0], false)]);
    assert_eq!(run.notify(KEYS_IN, &[0, 0, 0x04, 0, 0, 0, 0, 0]), [], "the Report is off");
    assert_eq!(run.notify(BOOT_IN, &[0, 0, 0x39, 0, 0, 0, 0, 0]), [KeyDown(0x39)]);
    run.go();
    assert_eq!(run.peer.writes[2..], [(BOOT_OUT, vec![0x02], true)]);
}

#[test]
fn a_map_that_does_not_parse() {
    boot(&mut Run::new(report_keyboard(&[0xfe, 0x00, 0x00])));
    boot(&mut Run::new(report_keyboard(&KEYBOARD[..64])));
}

#[test]
fn a_map_too_long_is_not_read_past_the_bound() {
    let mut run = Run::new(report_keyboard(&[0x34; 600]));
    boot(&mut run);
    let blobs = run.sent.iter().filter(|p| p[0] == 0x0c).count();
    assert_eq!(blobs, MAP_MAX / 22, "one Read and Read Blobs up to the bound, no more");
}

#[test]
fn a_map_that_cannot_be_read() {
    let mut peer = Peripheral::new(report_keyboard(&KEYBOARD));
    peer.refuse_read = Some((MAP, READ_NOT_PERMITTED));
    boot(&mut Run::with(peer));
    let mut peer = Peripheral::new(report_keyboard(&KEYBOARD));
    peer.refuse_read = Some((MAP, INSUFFICIENT_AUTHENTICATION));
    failed(&mut Run::with(peer), HogpError::Security(INSUFFICIENT_AUTHENTICATION));
}
