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
//! Each codec vendor's own setup: Cirrus Logic's init sequences, IDT's port
//! The coefficients a Cirrus codec holds once set up, against cs420x.c's
//! own values (sound/hda/codecs/cirrus/cs420x.c at 21a7f44fd076) written
//! out here rather than taken from the driver's tables, so a wrong value
//! there is caught.

use crate::proofs::fixtures::{alc236_hp, Desc};
use crate::proofs::program_sim::programmed;

fn coefs(vendor: u32, nid: u8, want: &[(u16, u16)]) {
    let (s, _sent) =
        programmed(&Desc { vendor, subsystem: 0x106b_7100, gpio: 4, ..alc236_hp() }, false);
    let held = s.coefs.lock().unwrap();
    for &(idx, value) in want {
        assert_eq!(held.get(&(0, nid, idx)), Some(&value), "{vendor:#x} coef {idx:#x}");
    }
}

#[test]
fn a_cs4206_holds_the_errata_and_init_values() {
    // :180-183 beep, :177-178 ADC, :169-175 DAC (2a | 40 | 1000 | 400), and
    // the errata's last writes at :228-233.
    let want = [(0x0004, 0x0007), (0x0002, 0x000a), (0x0003, 0x146a)];
    coefs(
        0x1013_4206,
        0x11,
        &[&want[..], &[(0x0017, 0x2412), (0x0008, 0x0000), (0x0001, 0x0008)]].concat(),
    );
}

#[test]
fn a_cs4208_holds_its_two() {
    coefs(0x1013_4208, 0x24, &[(0x0033, 0x0001), (0x0034, 0x1c01)]);
}
