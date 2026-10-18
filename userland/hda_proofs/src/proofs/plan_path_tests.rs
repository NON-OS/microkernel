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
//! The path chosen to each output: which mixer input it records, which DAC
//! it settles on, and which input a selector two outputs share is set to.

use crate::controller::codec::path::find;
use crate::controller::codec::pincfg::OutKind;
use crate::proofs::fixtures::{
    alc236_hp, alc269_mixers, codec, dac, pin, selector, Desc, PIN_OUT_WCAPS,
};
use crate::proofs::plan_tests::analog;

#[test]
fn a_path_through_a_mixer_records_which_mixer_input_the_dac_is_on() {
    let p = analog(&alc269_mixers());
    let spk = p.iter().find(|o| o.kind == OutKind::Speaker).unwrap();
    assert_eq!(&spk.path.nodes[..3], &[0x14, 0x0c, 0x02]);
    assert_eq!(&spk.path.sel[..2], &[0, 0], "the DAC is input 0 of mixer 0x0c");
}

#[test]
fn a_dac_that_cannot_play_48k_16_bit_is_passed_over() {
    let mut d = alc236_hp();
    d.widgets[0] = dac(0x02, 0);
    let p = analog(&d);
    for o in p.iter() {
        assert_eq!(o.path.dac(), 0x03);
        assert_eq!(o.path.sel[0], 1, "the pin must select its second input");
    }
}

#[test]
fn two_outputs_sharing_a_selector_agree_on_its_input() {
    let d = Desc {
        vendor: 0x10ec_0662,
        subsystem: 0,
        gpio: 0,
        widgets: vec![
            dac(0x02, 0x0002_0040),
            dac(0x03, 0x0002_0040),
            selector(0x0f, &[0x02, 0x03]),
            pin(0x14, PIN_OUT_WCAPS, 0x14, 0x9017_0110, &[0x0f]),
            pin(0x15, PIN_OUT_WCAPS, 0x1c, 0x0221_1020, &[0x0f]),
        ],
    };
    let c = codec(&d, 0);
    let spk = find(&c, 0x14, &[]).unwrap();
    let hp = find(&c, 0x15, &[spk]).unwrap();
    assert_eq!((spk.sel[1], hp.sel[1]), (0, 0), "the shared selector was asked for two inputs");
}
