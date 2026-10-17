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
//! Undo checks: an owner that was only slow hears LOST once and has its
//! screen back with its next frame; one heard again is watched afresh.

use super::ask::{ask, OWNER};
use super::draw::{draw, fill, present};
use super::rig::rig;
use crate::death::watch::watch;
use crate::protocol::{HEALTH_LOST, OP_HEALTHCHECK};

const NEW: u32 = 0x00C0_FFEE;

fn health(r: &super::rig::Rig, pid: u32) -> u32 {
    let (status, body) = ask(r, pid, OP_HEALTHCHECK, &[], 0);
    assert_eq!(status, 0);
    u32::from_le_bytes(body[..4].try_into().unwrap())
}

#[test]
fn a_slow_owner_hears_lost_once_and_its_next_frame_is_shown() {
    let r = rig();
    draw(&r);
    assert!(watch(&r.d, 3_000, true));
    assert_eq!(health(&r, OWNER + 1), 0, "LOST is the owner's alone");
    assert_eq!(health(&r, OWNER), HEALTH_LOST, "LOST once");
    assert_eq!(health(&r, OWNER), 0, "and only once");
    fill(&r, NEW);
    present(&r, OWNER, 3_100);
    let shown = r.gpu.borrow().shown(0).unwrap().iter().all(|&p| p == NEW);
    assert!(shown, "its new frame on scanout 0 again");
}

#[test]
fn an_owner_heard_again_is_watched_afresh() {
    let r = rig();
    draw(&r);
    r.d.owner.heard(OWNER + 1, 2_000);
    assert!(watch(&r.d, 3_000, true), "another pid is not the owner heard");
    r.d.owner.heard(OWNER, 3_100);
    assert!(!watch(&r.d, 3_200, true) && !watch(&r.d, 6_099, true), "heard at 3.1 s");
    assert!(watch(&r.d, 6_100, true), "a second silence blanks again");
    assert!(!watch(&r.d, 9_000, true), "once for each silence");
}
