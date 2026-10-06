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
//! Reading a codec into a description, and stopping on one that lies.

use nonos_devmodel::run;

use crate::controller::codec::pincfg::PinConfig;
use crate::controller::codec::walk::{find_afg, walk, FG_CAP};
use crate::model::{corb_engine, rings, window};
use crate::proofs::fixtures::{alc236_hp, sim_codec};
use crate::proofs::verb_tests::link_over;
use crate::sim;

#[test]
fn the_walk_reads_back_the_codec_as_it_describes_itself() {
    let d = alc236_hp();
    let s = sim::start(vec![(0, sim_codec(&d))]);
    let mut link = s.link();
    let c = walk(&mut link, 0).unwrap().expect("an audio function group");
    assert_eq!((c.vendor_id, c.subsystem_id, c.afg, c.gpio_count), (0x10ec_0236, 0x103c_8651, 1, 3));
    for want in &d.widgets {
        let got = c.get(want.nid).expect("widget missing from the walk");
        assert_eq!(got, want, "widget {:#x} read back differently", want.nid);
    }
    assert_eq!(c.get(0x14).unwrap().pin_cfg, PinConfig(0x9017_0110));
}

#[test]
fn a_codec_claiming_255_function_groups_is_asked_about_no_more_than_the_cap() {
    let bar = window();
    let (corb, rirb) = rings(0x0002_00ff);
    let _controller = run(&bar, corb_engine);
    let mut link = link_over(&bar, corb.base(), rirb.base());
    assert_eq!(find_afg(&mut link, 0), Ok(None));
    assert_eq!(bar.wrote16(crate::constants::CORBWP as usize), 1 + FG_CAP as u16, "the walk did not stop at the cap");
}
