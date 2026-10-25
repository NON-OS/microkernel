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
//! A silent owner's blank (G-72's, run by the wire server) gone wrong at
//! every command: the primaries are zeroed and flushed twice, so the host's
//! copy and the screen hold none of the owner's frame once a transfer has
//! landed, and a host back again has the next wake finish the blank (each
//! wake a turn of the serve loop, which settles the queue first), the
//! clearing said once however many wakes it takes.

use nonos_gfx_serve::OWNER_SILENCE_MS;

use super::kit::{draw, shown};
use super::rig::rig;
use super::wrong::{run, wrongs, Wrong};
use crate::constants::VG_CMD_TRANSFER_TO_HOST_2D;
use crate::server::recover::settle;

const SECRET: u32 = 0x5EC2_E7ED;

#[test]
fn a_silent_owner_s_blank_wrong_at_any_command_leaves_none_of_its_frame() {
    let n = {
        let mut r = rig();
        draw(&mut r, 0, SECRET);
        r.d.server.heard(7, 0);
        let from = r.host.borrow().steps();
        assert!(r.d.server.watch(OWNER_SILENCE_MS));
        let to = r.host.borrow().steps();
        to - from
    };
    assert!(n >= 4, "{n} commands in a blank");
    for w in wrongs(n) {
        let mut r = rig();
        draw(&mut r, 0, SECRET);
        r.d.server.heard(7, 0);
        let from = r.host.borrow().steps() as usize;
        let done = run(&mut r, w, |r| r.d.server.watch(OWNER_SILENCE_MS));
        let sent = r.host.borrow().log[from..]
            .iter()
            .any(|c| c.landed && c.kind == VG_CMD_TRANSFER_TO_HOST_2D);
        let secret = shown(&r, 0).unwrap().contains(&SECRET);
        assert!(!sent || !secret, "{w:?} of {n}: none of it once a transfer landed");
        let Wrong::Dead(k, d) = w else { continue };
        assert_eq!(done, Some(true), "{w:?}");
        let mut told = r.d.server.take_tell() as u32;
        if d > 0 || k > n {
            let mut t = OWNER_SILENCE_MS;
            while shown(&r, 0).unwrap().contains(&SECRET) && t < OWNER_SILENCE_MS + 2_500 {
                t += 500;
                settle(&mut r.d);
                r.d.server.watch_said(t, |_| told += 1);
            }
            assert!(shown(&r, 0).unwrap().iter().all(|&p| p == 0), "{w:?}: blank at last");
        }
        assert_eq!(told, 1, "{w:?}: the clearing said once, retries or not");
    }
}
