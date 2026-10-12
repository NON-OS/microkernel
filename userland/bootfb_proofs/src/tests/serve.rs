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

//! Driving the boot framebuffer through the wire server.

use crate::display::BootFb;
use crate::fb::{Fb, Layout};
use crate::pool::Pool;
use nonos_gfx_serve::Server;
use nonos_gfx_wire::{ops, Header, Reply, Surface, HDR_LEN};
use std::sync::atomic::{AtomicU32, Ordering};

pub static FENCES: AtomicU32 = AtomicU32::new(0);

pub fn ask(s: &mut Server<BootFb, Pool>, op: u16, payload: &[u8]) -> (i32, Vec<u8>) {
    let h = Header { op, flags: 0, output: 0, request_id: 1, payload_len: payload.len() as u32 };
    let mut f = vec![0u8; HDR_LEN];
    h.encode(&mut f).unwrap();
    f.extend_from_slice(payload);
    let mut out = [0u8; 300];
    let n = s.handle(7, &f, &mut out);
    let r = Reply::decode(&out[..n], &h).unwrap();
    (r.status, r.body.to_vec())
}

pub fn words(v: &[u32]) -> Vec<u8> {
    v.iter().flat_map(|x| x.to_le_bytes()).collect()
}

pub fn run(layout: Layout, fb_mem: &mut Vec<u32>) -> (Server<BootFb, Pool>, Surface) {
    let fb =
        Fb::new(fb_mem.as_mut_ptr() as *mut u8, fb_mem.len() * 4, (16, 8), 80, layout).unwrap();
    let mut s = Server::new(
        BootFb {
            fb,
            fence: || {
                FENCES.fetch_add(1, Ordering::SeqCst);
            },
        },
        Pool::default(),
    );
    let sf = Surface::decode(&ask(&mut s, ops::SURFACE, &[]).1).unwrap();
    let base = s.current.unwrap().region.base;
    for px in s.platform.memory.get_mut(&base).unwrap().chunks_exact_mut(4) {
        px.copy_from_slice(&0xFF11_2233u32.to_le_bytes());
    }
    (s, sf)
}
