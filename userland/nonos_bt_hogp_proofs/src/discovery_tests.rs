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

//! Discovery and configuration against whole devices: the right handles
//! found, boot mode set with a Write Command, and notifications turned on
//! with Write Requests to the CCCDs (Vol 3, Part G, 4.4.2 to 4.12.3).

use crate::sim::{combo, keyboard_mouse, Run};
use nonos_bt_hogp::host::{Kind, Outcome, Phase};
use nonos_bt_hogp::report::Input;

#[test]
fn a_keyboard_with_a_pointer_is_found_and_configured() {
    let db = keyboard_mouse();
    let (mode, kb, mouse) = (db.handle_of(0x2a4e), db.handle_of(0x2a22), db.handle_of(0x2a33));
    let mut run = Run::new(db);
    assert_eq!(run.go(), Some(Outcome::Ready { keyboard: true, mouse: true }));
    assert_eq!(run.host.phase(), Phase::Ready);
    let s = run.host.sources();
    assert_eq!((s[0].kind, s[0].value, s[0].cccd), (Kind::Keyboard, kb, Some(kb + 1)));
    // The Report Reference sits between the mouse value and its CCCD.
    assert_eq!((s[1].kind, s[1].value, s[1].cccd), (Kind::Mouse, mouse, Some(mouse + 2)));
    let w = &run.peer.writes;
    assert_eq!(w[0], (mode, vec![0], true));
    assert_eq!(w[1], (kb + 1, vec![1, 0], false));
    assert_eq!(w[2], (mouse + 2, vec![1, 0], false));
    assert_eq!(w.len(), 3);
    assert_eq!(run.notify(kb, &[0, 0, 0x04, 0, 0, 0, 0, 0]), [Input::KeyDown(0x04)]);
    assert_eq!(
        run.notify(mouse, &[1, 5, 0xfb]),
        [Input::Motion { dx: 5, dy: -5 }, Input::ButtonDown(1)]
    );
    assert_eq!(run.host.disconnected(), [Input::KeyUp(0x04), Input::ButtonUp(1)]);
}

#[test]
fn a_combined_device_with_two_hid_services() {
    let db = combo();
    let (kb, mouse) = (db.handle_of(0x2a22), db.handle_of(0x2a33));
    let mut run = Run::new(db);
    assert_eq!(run.go(), Some(Outcome::Ready { keyboard: true, mouse: true }));
    let modes: Vec<_> =
        run.peer.writes.iter().filter(|w| w.2).map(|w| (w.0, w.1.clone())).collect();
    assert_eq!(modes, [(3, vec![0]), (9, vec![0])]);
    assert_eq!(run.notify(kb, &[0x02, 0, 0, 0, 0, 0, 0, 0]), [Input::KeyDown(0xe1)]);
    assert_eq!(run.notify(mouse, &[0, 0, 0]), []);
}
