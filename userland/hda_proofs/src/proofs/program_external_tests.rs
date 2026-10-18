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

//! What the HP 15s's ALC236 needs beyond its paths, read back off the
//! ring: EAPD, the Realtek coefficients, the HP GPIOs, and the converter
//! joined to the stream with the format SDnFMT carries.

use crate::constants::{
    VERB_SET_CHANNEL_STREAMID, VERB_SET_EAPD_BTLENABLE, VERB_SET_GPIO_DATA,
    VERB_SET_GPIO_DIRECTION, VERB_SET_GPIO_MASK, VERB_SET_STREAM_FORMAT,
};
use crate::controller::codec::format::PLAYBACK;
use crate::controller::codec::program::hp_gpio;
use crate::controller::compose_verb_long;
use crate::proofs::fixtures::alc236_hp;
use crate::proofs::program_sim::{at, programmed, short, TAG};

#[test]
fn eapd_is_raised_on_every_pin_that_has_it() {
    let (_s, sent) = programmed(&alc236_hp(), false);
    for nid in [0x14u8, 0x1b, 0x21] {
        at(&sent, short(nid, VERB_SET_EAPD_BTLENABLE, 0x02));
    }
}

#[test]
fn the_realtek_eapd_coefficient_and_alc256_init_are_applied() {
    let (s, _sent) = programmed(&alc236_hp(), false);
    let coefs = s.coefs.lock().unwrap();
    assert_eq!(coefs.get(&(0, 0x20, 0x10)), Some(&0x0020), "coef 0x10 bit 9 still set");
    assert_eq!(coefs.get(&(0, 0x20, 0x36)), Some(&0x5757));
    assert_eq!(
        coefs.get(&(0, 0x57, 0x04)).map(|v| v & 7),
        Some(4),
        "headphone amp left in low power"
    );
}

#[test]
fn an_hp_machine_gets_its_gpios_driven_high_as_outputs() {
    let (_s, sent) = programmed(&alc236_hp(), false);
    let m = at(&sent, short(1, VERB_SET_GPIO_MASK, 0x07));
    let d = at(&sent, short(1, VERB_SET_GPIO_DIRECTION, 0x07));
    let v = at(&sent, short(1, VERB_SET_GPIO_DATA, 0x07));
    assert!(m < d && d < v, "mask, direction, then data, as Linux writes them");
    assert_eq!(hp_gpio(0x1025_0000, 3), None, "only HP machines");
    assert_eq!(hp_gpio(0x103c_8651, 0), None);
    assert_eq!(hp_gpio(0x103c_8651, 9), Some(0xff));
}

#[test]
fn the_converter_takes_the_stream_tag_and_the_format_sdnfmt_carries() {
    let (_s, sent) = programmed(&alc236_hp(), false);
    let f = at(&sent, compose_verb_long(0, 0x02, VERB_SET_STREAM_FORMAT as u16, PLAYBACK));
    let t = at(&sent, short(0x02, VERB_SET_CHANNEL_STREAMID, (TAG as u16) << 4));
    assert!(f < t, "the stream joined before the format was set");
    let joins = sent.iter().filter(|&&c| c == short(0x02, VERB_SET_CHANNEL_STREAMID, 0x10)).count();
    assert_eq!(joins, 1, "a DAC shared by two outputs was joined twice");
}
