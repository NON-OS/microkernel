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

//! The owner's silence: a compositor not heard for OWNER_SILENCE_MS
//! has its cursor turned off, every primary scrubbed and the screen shown
//! blank from it, once for each silence; its next STATE says LOST. Another
//! pid's requests do not count as the owner heard.

use crate::model::wire::{ask, u32s, COMPOSITOR};
use crate::model::{Pool, Vram};
use crate::server::Server;
use crate::watch::OWNER_SILENCE_MS;
use nonos_gfx_wire::body::state::LOST;
use nonos_gfx_wire::{ops, Cursor, State, Surface};

fn shown() -> (Server<Vram, Pool>, Surface) {
    let mut s = Server::new(Vram::new(true, true), Pool::new(8192));
    let sf = Surface::decode(&ask(&mut s, COMPOSITOR, ops::SURFACE, 0, &[]).1).unwrap();
    s.heard(COMPOSITOR, 0);
    let base = s.slots[0].current.unwrap().region.base;
    s.platform.memory.get_mut(&base).unwrap().fill(0x5A);
    let whole = u32s(&[sf.serial, 1, 0, 0, 1024, 768]);
    assert_eq!(ask(&mut s, COMPOSITOR, ops::FLUSH, 0, &whole).0, 0);
    let c = Cursor { flags: 3, x: 5, y: 5, hot_x: 0, hot_y: 0, width: 64, height: 64 };
    assert_eq!(ask(&mut s, COMPOSITOR, ops::CURSOR, 0, &c.encode()).0, 0);
    (s, sf)
}

fn state(s: &mut Server<Vram, Pool>) -> State {
    State::decode(&ask(s, COMPOSITOR, ops::STATE, 0, &[]).1).unwrap()
}

#[test]
fn a_silent_owner_s_last_picture_and_cursor_are_gone_and_scrubbed() {
    let (mut s, _) = shown();
    s.heard(COMPOSITOR + 1, OWNER_SILENCE_MS);
    assert!(!s.watch(OWNER_SILENCE_MS - 1), "not yet silent long enough");
    assert!(s.display.fb[..1024 * 768].iter().all(|&p| p == 0x5A5A_5A5A));
    assert!(s.watch(OWNER_SILENCE_MS), "another pid heard is not the owner heard");
    assert!(s.display.fb[..1024 * 768].iter().all(|&p| p == 0), "the screen blank");
    assert_eq!(s.display.cursor.as_ref().map(|c| c.0.flags), Some(0), "the cursor off");
    let base = s.slots[0].current.unwrap().region.base;
    assert!(s.platform.memory[&base].iter().all(|&b| b == 0), "the primary scrubbed");
    assert!(!s.watch(OWNER_SILENCE_MS * 2), "once for each silence");
    assert_ne!(state(&mut s).flags & LOST, 0, "the next STATE says LOST");
    assert_eq!(state(&mut s).flags & LOST, 0, "and only that one");
}

#[test]
fn an_owner_heard_again_is_watched_afresh() {
    let (mut s, _) = shown();
    s.heard(COMPOSITOR, 2_000);
    assert!(!s.watch(4_000), "heard at 2 s: not silent at 4 s");
    assert!(s.watch(5_000));
    s.heard(COMPOSITOR, 6_000);
    assert!(!s.watch(8_999) && s.watch(9_000), "a second silence blanks again");
}

#[test]
fn a_server_with_no_owner_has_nothing_to_blank() {
    assert!(!Server::new(Vram::new(true, true), Pool::new(8192)).watch(u64::MAX));
}
