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
//! Finding out which codecs are actually there, at whatever address.
//!
//! The probe used the Immediate Command registers while the CORB was running,
//! which the specification does not allow and which some controllers do not
//! implement. It goes through the CORB now, to every address STATESTS names.

use crate::controller::probe;
use crate::proofs::fixtures::{alc236_hp, intel_hdmi, sim_codec};
use crate::sim;

#[test]
fn a_codec_at_a_nonzero_address_is_found_there() {
    let s = sim::start(vec![(2, sim_codec(&alc236_hp()))]);
    let mut link = s.link();
    let found = probe(&mut link, 1 << 2);
    assert_eq!((found[2].present, found[2].ok), (1, 1), "codec two never answered");
    assert_eq!((found[2].vendor_id, found[2].device_id), (0x10ec, 0x0236));
    assert_eq!(found[0].present, 0, "address zero was assumed");
}

#[test]
fn an_analog_codec_and_a_display_codec_are_both_identified() {
    let s = sim::start(vec![(0, sim_codec(&alc236_hp())), (2, sim_codec(&intel_hdmi()))]);
    let mut link = s.link();
    let found = probe(&mut link, 0b101);
    assert_eq!(found[0].vendor_id, 0x10ec);
    assert_eq!((found[2].vendor_id, found[2].device_id), (0x8086, 0x280d));
}

#[test]
fn an_absent_codec_is_left_alone_and_still_carries_its_own_address() {
    let s = sim::start(vec![]);
    let mut link = s.link();
    let found = probe(&mut link, 0);
    for (i, p) in found.iter().enumerate() {
        assert_eq!(p.address as usize, i, "slot {i} describes another address");
        assert_eq!((p.present, p.ok, p.vendor_id), (0, 0, 0), "slot {i} invented a codec");
    }
    assert!(s.sent().is_empty(), "a codec nobody reported was asked anyway");
}
