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

//! The owner's silence on the Intel engine (G-72): the picture the
//! compositor last flushed is gone from both surfaces and from the screen,
//! and the cursor plane is off.

use crate::engine::plane_regs::cur_ctl;
use crate::tests::wire_rig::{h0, rig};
use crate::tests::wire_sweep::heap::Heap;
use nonos_gfx_serve::{Display, Platform, Server, OWNER_SILENCE_MS};
use nonos_gfx_wire::{ops, Cursor, Header, Reply, Surface, HDR_LEN};

fn ask<D: Display, P: Platform>(s: &mut Server<D, P>, op: u16, p: &[u8]) -> i32 {
    let h = Header { op, flags: 0, output: 0, request_id: 1, payload_len: p.len() as u32 };
    let mut f = vec![0u8; HDR_LEN];
    h.encode(&mut f).unwrap();
    f.extend_from_slice(p);
    let mut out = [0u8; 300];
    let n = s.handle(7, &f, &mut out);
    let r = Reply::decode(&out[..n], &h).unwrap();
    if op == ops::SURFACE {
        return Surface::decode(r.body).unwrap().serial as i32;
    }
    r.status
}

#[test]
fn a_silent_owner_s_picture_leaves_both_surfaces_and_the_cursor_goes_off() {
    let (d, model) = rig();
    let mut s = Server::new(d, Heap::new(1 << 16));
    let serial = ask(&mut s, ops::SURFACE, &[]) as u32;
    s.heard(7, 0);
    let p = s.slots[0].current.unwrap();
    s.platform.memory.get_mut(&p.region.base).unwrap().fill(0x66);
    let w = |v: &[u32]| v.iter().flat_map(|x| x.to_le_bytes()).collect::<Vec<u8>>();
    let all = w(&[serial, 1, 0, 0, p.width, p.height]);
    assert_eq!((ask(&mut s, ops::FLUSH, &all), ask(&mut s, ops::FLUSH, &all)), (0, 0));
    let c = Cursor { flags: 3, x: 10, y: 10, hot_x: 0, hot_y: 0, width: 64, height: 64 };
    assert_eq!(ask(&mut s, ops::CURSOR, &c.encode()), 0);
    let f = &h0(&s.display).frames;
    let (pipe, iova, len) = (h0(&s.display).out.pipe, f.iova, f.len as u64);
    assert_eq!(model.borrow().pixel(pipe, 5, 5), Some(0x6666_6666), "shown");
    assert!(s.watch(OWNER_SILENCE_MS));
    let m = model.borrow();
    assert_eq!((m.pixel(pipe, 5, 5), m.pixel(pipe, p.width - 1, p.height - 1)), (Some(0), Some(0)));
    let page = |a: u64| m.memory.get(&a).is_none_or(|pg| pg.iter().all(|&b| b == 0));
    let pages = |a: u64| (0..len.div_ceil(4096)).all(|i| page(a + i * 4096));
    assert!(iova.into_iter().all(pages), "both surfaces scrubbed, every page");
    assert_eq!(m.get(cur_ctl(pipe)) & 0x27, 0, "the cursor plane off");
}
