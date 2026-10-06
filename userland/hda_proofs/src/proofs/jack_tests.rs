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
//! Speaker or headphones, by the jack.

use std::sync::atomic::Ordering;

use crate::constants::{PIN_HP_ENABLE, PIN_OUT_ENABLE, VERB_SET_PIN_WIDGET_CONTROL};
use crate::controller::codec::jack::{apply, pin_ctl, plugged};
use crate::controller::codec::pincfg::OutKind;
use crate::controller::codec::plan::{plan, Class};
use crate::controller::compose_verb;
use crate::proofs::fixtures::{alc236_hp, codec, qemu_duplex, sim_codec};
use crate::sim;

#[test]
fn the_speakers_play_only_while_nothing_is_plugged_in() {
    assert_eq!(pin_ctl(OutKind::Speaker, false, false), PIN_OUT_ENABLE);
    assert_eq!(pin_ctl(OutKind::Speaker, false, true), 0);
    assert_eq!(pin_ctl(OutKind::Headphone, true, true), PIN_OUT_ENABLE | PIN_HP_ENABLE);
    assert_eq!(pin_ctl(OutKind::Headphone, false, false), PIN_OUT_ENABLE);
    assert_eq!(pin_ctl(OutKind::LineOut, false, true), PIN_OUT_ENABLE, "line out is not auto-muted");
}

#[test]
fn plugging_headphones_in_is_read_from_the_jack_and_switches_the_speaker_off() {
    let d = alc236_hp();
    let s = sim::start(vec![(0, sim_codec(&d))]);
    let mut link = s.link();
    let c = codec(&d, 0);
    let Class::Analog(p) = plan(&c) else { panic!() };
    assert_eq!(plugged(&mut link, &c, &p), Ok(false));
    s.plugged.store(true, Ordering::Release);
    assert_eq!(plugged(&mut link, &c, &p), Ok(true));
    apply(&mut link, &c, &p, true).unwrap();
    assert!(s.sent().contains(&compose_verb(0, 0x14, VERB_SET_PIN_WIDGET_CONTROL, 0)));
}

#[test]
fn a_codec_without_speakers_never_reads_as_plugged() {
    let d = qemu_duplex();
    let s = sim::start(vec![(0, sim_codec(&d))]);
    s.plugged.store(true, Ordering::Release);
    let mut link = s.link();
    let c = codec(&d, 0);
    let Class::Analog(p) = plan(&c) else { panic!() };
    assert_eq!(plugged(&mut link, &c, &p), Ok(false));
    assert!(s.sent().is_empty(), "a jack was read with nothing to switch");
}
