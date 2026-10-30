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

//! The owner's silence on the DISPI adapter: the picture the
//! compositor last flushed is gone from video memory, which outlives any
//! process, not left there.

use super::wire_sweep::heap::Heap;
use crate::dispi::DispiMode;
use crate::edid::Panel;
use crate::model::Adapter;
use nonos_gfx_serve::{Display, Platform, Server, OWNER_SILENCE_MS};
use nonos_gfx_wire::{ops, Header, Reply, Surface, HDR_LEN};

pub fn ask<D: Display, P: Platform>(s: &mut Server<D, P>, op: u16, p: &[u8]) -> (i32, Vec<u8>) {
    let h = Header { op, flags: 0, output: 0, request_id: 1, payload_len: p.len() as u32 };
    let mut f = vec![0u8; HDR_LEN];
    h.encode(&mut f).unwrap();
    f.extend_from_slice(p);
    let mut out = [0u8; 300];
    let n = s.handle(7, &f, &mut out);
    let r = Reply::decode(&out[..n], &h).unwrap();
    (r.status, r.body.to_vec())
}

pub fn words(v: &[u32]) -> Vec<u8> {
    v.iter().flat_map(|x| x.to_le_bytes()).collect()
}

#[test]
fn a_silent_owner_s_picture_is_wiped_from_video_memory() {
    let mut a = Adapter::new();
    let b = a.bga(Panel::default(), DispiMode { width: 1024, height: 768, stride: 4096 });
    let mut s = Server::new(b, Heap::new(8192));
    let sf = Surface::decode(&ask(&mut s, ops::SURFACE, &[]).1).unwrap();
    s.heard(7, 0);
    let base = s.slots[0].current.unwrap().region.base;
    s.platform.memory.get_mut(&base).unwrap().fill(0x44);
    assert_eq!(ask(&mut s, ops::FLUSH, &words(&[sf.serial, 1, 0, 0, 1024, 768])).0, 0);
    assert!(a.vram[..1024 * 768 * 4].iter().all(|&b| b == 0x44), "shown");
    assert!(s.watch(OWNER_SILENCE_MS));
    assert!(a.vram[..1024 * 768 * 4].iter().all(|&b| b == 0), "wiped");
}
