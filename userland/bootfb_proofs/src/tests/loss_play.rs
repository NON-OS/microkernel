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
//! One order of events played against bootfb as its serve loop drives it
//!, the kernel taking the scanout where the order says, and judged
//! at every step (`loss_tests`).

use super::loss_ask::{fence, look, present, MID};
use super::serve::{ask, board, HELD, RELEASED};
use crate::fb::{Fb, Layout};
use crate::pool::Pool;
use nonos_gfx_serve::Server;
use nonos_gfx_wire::{ops, Surface};

#[derive(Clone, Copy, Debug)]
pub enum Ev {
    Present(u32),
    Idle,
    Look,
    /// The kernel takes the scanout now, or in the middle of the next present.
    Take,
    TakeMid,
}

pub fn play(evs: &[Ev]) {
    MID.with(|m| m.set(false));
    let mut mem = vec![0u32; 16 * 8 * 2];
    let fb = Fb::new(mem.as_mut_ptr() as *mut u8, mem.len() * 4, (16, 8), 64, Layout::BlueFirst);
    let mut s = Server::new(board(fb.unwrap(), fence), Pool::default());
    let sf = Surface::decode(&ask(&mut s, ops::SURFACE, &[]).1).unwrap();
    let (mut told, mut seen) = (0, None::<Vec<u32>>);
    for (i, &ev) in evs.iter().enumerate() {
        // Before every frame and on every wake, as the serve loop asks.
        told += s.display.tell_due() as u32;
        if s.display.gone && seen.is_none() {
            seen = Some(mem.clone());
        }
        let tag = format!("{evs:?} at {i}");
        match ev {
            Ev::Present(px) => present(&mut s, sf, px, &seen, &tag),
            Ev::TakeMid => {
                MID.with(|m| m.set(true));
                present(&mut s, sf, 0x00AB_0000 | i as u32, &seen, &tag);
            }
            Ev::Idle => {
                let _ = s.watch(1_000 * i as u64);
            }
            Ev::Look => look(&mut s, &seen, &tag),
            Ev::Take => HELD.with(|h| h.set(false)),
        }
        if let Some(snap) = &seen {
            assert_eq!(&mem, snap, "{tag}: drawn after the scanout was lost");
        }
    }
    let lost = !HELD.with(|h| h.get());
    told += s.display.tell_due() as u32;
    assert_eq!((told, RELEASED.with(|r| r.get())), (lost as u32, lost as u32), "{evs:?}");
}
