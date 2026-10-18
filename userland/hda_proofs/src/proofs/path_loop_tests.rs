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

//! The other side of the search budget: a graph that loops but is small
//! ends as no route, never as a spent budget; no real codec's pin comes near
//! the budget; and a bring-up names a codec whose graph spent it, and goes
//! on to the verdict rather than failing the controller.

use crate::constants::WIDGET_TYPE_PIN;
use crate::controller::codec::path::{find, Miss};
use crate::controller::verdict::Verdict;
use crate::proofs::fixtures::{alc236_hp, alc269_mixers, codec, intel_hdmi, mixer, pin};
use crate::proofs::fixtures::{qemu_duplex, sim_codec, Desc, PIN_OUT_WCAPS};
use crate::proofs::lifecycle::attempt::attempt_until;
use crate::proofs::lifecycle::board::{board, LOW};
use crate::proofs::lifecycle::given_back::quiesced_then_released;
use crate::proofs::path_budget_tests::maze;
use crate::setup::Started;

const NO_OUTPUT_PATH: u32 = Verdict::NoOutputPath as u32;

#[test]
fn a_loop_of_mixers_with_no_dac_ends_as_no_route() {
    let widgets = vec![
        mixer(0x20, &[0x20, 0x21, 0x22]),
        mixer(0x21, &[0x22, 0x20, 0x21]),
        mixer(0x22, &[0x21, 0x20, 0x22]),
        pin(0x14, PIN_OUT_WCAPS, 0x10, 0x9017_0110, &[0x20, 0x21, 0x22]),
    ];
    let d = Desc { vendor: 0x1af4_0022, subsystem: 0, gpio: 0, widgets };
    assert_eq!(find(&codec(&d, 0), 0x14, &[]), Err(Miss::NoRoute));
}

#[test]
fn no_real_codec_spends_the_budget_on_any_pin() {
    for d in [alc236_hp(), alc269_mixers(), qemu_duplex(), intel_hdmi()] {
        let c = codec(&d, 0);
        for w in c.all().iter().filter(|w| w.ty() == WIDGET_TYPE_PIN) {
            let r = find(&c, w.nid, &[]);
            assert_ne!(r, Err(Miss::OverBudget), "{:#x} pin {:#x}", d.vendor, w.nid);
        }
    }
}

#[test]
fn a_bring_up_names_a_codec_whose_graph_spent_the_budget_and_goes_on() {
    let maze = || board(vec![(0, sim_codec(&maze(false)))], &LOW);
    // A run whose codec model was starved ends silent without the line.
    let named = |r: &_, l: &[String]| {
        let silent = matches!(r, Ok(Started::Silent(s)) if s.verdict == NO_OUTPUT_PATH);
        silent && l.iter().any(|l| l == "[HDA] codec 0 graph over budget\n")
    };
    let (_b, _, log) = attempt_until(maze, named);
    quiesced_then_released(&log);
}
