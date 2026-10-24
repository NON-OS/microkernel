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

//! The start-up mode from any register file an adapter might leave, any
//! monitor's preferred mode and any mapping: exactly what the oracle says,
//! a refusal writing no register, and a mode set the one the device shows.

use std::cell::Cell;
use std::rc::Rc;

use nonos_libc::on_w16;

use super::file::{draw, window};
use super::lie::adapter;
use super::oracle::{want, Want};
use crate::dispi::current;
use crate::edid::Panel;
use crate::regs::Regs;
use crate::setup::mode::starting;
use crate::tests::wire_sweep::rng::Rng;

#[test]
fn any_register_file_starts_on_the_mode_the_rules_say() {
    let (mut r, mut tally) = (Rng(0x6267_6101), [0u32; 4]);
    for i in 0..5_000 {
        let f = draw(&mut r);
        let sides = [640, 800, 1024, 1280, 1366, 1920, 2560, 4096, 4104, 1004, 632, 472, 320, 0];
        let pick = |r: &mut Rng| sides[r.below(sides.len() as u32) as usize];
        let preferred = (r.below(3) != 0).then(|| (pick(&mut r), pick(&mut r)));
        let mapped = [0u64, 1 << 20, 3 << 20, 8 << 20, 16 << 20, 1 << 32][r.below(6) as usize];
        let lie = r.below(6) == 0;
        let (bar, writes) = (window(&f), Rc::new(Cell::new(0)));
        adapter(&bar, lie, Rc::clone(&writes));
        let regs = Regs::new(bar.base());
        let panel = Panel { preferred, ..Panel::default() };
        let video = u64::from(f[9]) * 0x1_0000;
        let got = starting(regs, &panel, (video, mapped));
        on_w16(|_, _| {});
        let w = want(&f, preferred, mapped, lie);
        match (&w, got) {
            (Want::Kept(w, h, s), Ok(m)) => {
                assert_eq!((m.width, m.height, m.stride, writes.get()), (*w, *h, *s, 0), "case {i}")
            }
            (Want::Set(w, h), Ok(m)) => {
                assert_eq!((m.width, m.height, m.stride), (*w, *h, w * 4), "case {i}");
                assert_eq!(current(regs), Some(m), "case {i}: not what the device shows");
            }
            (Want::Refused(e), Err(g)) => assert_eq!((*e, writes.get()), (g, 0), "case {i}"),
            (Want::NotTaken, Err(g)) => {
                assert_eq!(g, crate::error::BgaError::ModeNotTaken, "case {i}")
            }
            (w, g) => panic!("case {i}: wanted {w:?}, got {g:?} from {f:?} {preferred:?} {mapped}"),
        }
        tally[match w {
            Want::Kept(..) => 0,
            Want::Set(..) => 1,
            Want::Refused(_) => 2,
            Want::NotTaken => 3,
        }] += 1;
    }
    assert!(tally.iter().all(|&n| n >= 200), "a verdict barely tried: {tally:?}");
}
