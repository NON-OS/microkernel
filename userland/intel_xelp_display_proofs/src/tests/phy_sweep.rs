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
//! Random signals, levels as a hostile sink could ask for them in link
//! training: each is refused with nothing written, or programmed with every
//! lane holding the fixed fields Vol 12 sets (p. 393) and training on.

use crate::model::regs::Regs;
use crate::model::rng::Rng;
use nonos_intel_xelp_display::phy::signal::Signal;
use nonos_intel_xelp_display::phy::swing::program;
use nonos_intel_xelp_display::regs::clka::Combo;

const RATES: [u32; 8] = [1620, 2160, 2700, 3240, 4320, 5400, 6480, 8100];

fn signal(r: &mut Rng) -> Signal {
    let rate = if r.word().is_multiple_of(4) { r.word() } else { RATES[(r.word() % 8) as usize] };
    let (lanes, vs, pe) = (r.word() % 6, r.word() % 6, r.word() % 6);
    match r.word() % 3 {
        0 => Signal::Dp { mbps: rate, lanes, vs, pe },
        1 => Signal::Edp { mbps: rate, lanes, vs, pe },
        _ => {
            Signal::Hdmi { row: (r.word() % 9) as usize, symbol_khz: r.field().saturating_mul(100) }
        }
    }
}

#[test]
fn every_signal_refused_cleanly_or_fully_programmed() {
    let mut r = Rng(0xD1B5_4A32_D192_ED03);
    let (mut ok, mut no) = (0, 0);
    for _ in 0..20_000 {
        let sig = signal(&mut r);
        let mut m = Regs::new(Some(0));
        if program(&mut m, Combo::A, sig).is_err() {
            assert!(m.writes.is_empty(), "{sig:?}");
            no += 1;
            continue;
        }
        ok += 1;
        for n in 0..4 {
            let at = |off: u32| m.get(0x162800 + 0x100 * n + off);
            // Rcomp 98h; DW5 training, 3tap off, scaling 010b, rterm 110b.
            assert_eq!(at(0x88) & 0xFF, 0x98);
            let dw5 = at(0x94);
            assert_eq!(dw5 & !(1 << 30), (1 << 31) | (1 << 29) | (0b010 << 18) | (0b110 << 3));
            // Post Cursor 2 zero; loadgen never on lane 0 (p. 392).
            assert_eq!(at(0x90) & (0x3F << 6), 0);
            assert!(n != 0 || at(0x90) >> 31 == 0);
        }
        let last = *m.writes.last().unwrap();
        assert_eq!(last.0, 0x162694);
    }
    assert!(ok > 1000 && no > 1000, "{ok} {no}");
}
