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
//! A resource the host would not let go of in a blank is owned by no one
//! from then on: its old owner's requests on it are refused, and the next
//! blank tries it again.

use super::ask::{ask, words, OWNER};
use super::draw::draw;
use super::judge::landed;
use super::kit::{run, Wrong};
use super::rig::rig;
use crate::constants::VG_CMD_RESOURCE_UNREF;
use crate::death::watch::watch;
use crate::protocol::{E_BUSY, OP_FLUSH};

#[test]
fn a_resource_the_host_keeps_is_its_owner_s_no_longer() {
    let probe = rig();
    let own = draw(&probe);
    let from = probe.gpu.borrow().steps();
    watch(&probe.d, 100, false);
    let at = probe.gpu.borrow().log[from as usize..]
        .iter()
        .position(|c| c.kind == VG_CMD_RESOURCE_UNREF);
    drop(probe);
    let r = rig();
    assert_eq!(draw(&r), own);
    let from = r.gpu.borrow().steps();
    run(&r, Wrong::Dead(at.unwrap() as u32 + 1, 0), || watch(&r.d, 100, false));
    assert!(
        !landed(&r, from, VG_CMD_RESOURCE_UNREF, own, 0) && r.gpu.borrow().res.contains_key(&own)
    );
    let rect = words(&[own, 0, 0, 8, 8]);
    assert_eq!(ask(&r, OWNER, OP_FLUSH, &rect, 200).0, E_BUSY, "owned by no one");
}
