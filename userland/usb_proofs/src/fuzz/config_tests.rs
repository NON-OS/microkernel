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

//! Two hundred thousand hostile configuration descriptors each, structured
//! and arbitrary, through the driver's walk: nothing may panic, and every
//! walk must pick what the reference rules pick.

use super::hostile::hostile_config;
use super::reference::reference;
use super::ROUNDS;
use crate::config_blob::summaries;
use crate::descriptors::hid_bindings;
use crate::descriptors::types::HidKind;
use crate::protocol::MAX_HID_BINDINGS;
use crate::rng::Rng;

/// Any byte string, sometimes behind a header that lets the walk start.
pub fn hostile_bytes(rng: &mut Rng) -> Vec<u8> {
    let len = rng.below(600);
    let mut d = rng.bytes(len);
    if len >= 9 && rng.one_in(2) {
        d[0] = 9;
        d[1] = 0x02;
        let total = if rng.one_in(2) { len as u16 } else { rng.word() as u16 };
        d[2..4].copy_from_slice(&total.to_le_bytes());
    }
    d
}

fn check_config(raw: &[u8], round: u32) {
    let got = hid_bindings(raw);
    assert_eq!(got.as_deref().ok().map(summaries), reference(raw), "round {round}: {raw:02x?}");
    let Ok(bindings) = got else { return };
    assert!(bindings.len() <= MAX_HID_BINDINGS, "round {round}");
    for b in &bindings {
        let least = match b.kind {
            HidKind::Keyboard => 8,
            HidKind::Mouse => 3,
            HidKind::Report => 1,
            HidKind::Gamepad => panic!("round {round}: a configuration alone made a gamepad"),
        };
        assert!(b.endpoint_address & 0x80 != 0, "round {round}: not IN");
        assert!((1..=15).contains(&(b.endpoint_address & 0x0f)), "round {round}: endpoint 0");
        assert!((least..=1024).contains(&b.max_packet_size), "round {round}: {b:?}");
    }
}

#[test]
fn hostile_configuration_descriptors_bind_only_what_the_rules_allow() {
    let mut rng = Rng::seeded(0x5eed_0001);
    for round in 0..ROUNDS {
        check_config(&hostile_config(&mut rng), round);
    }
}

#[test]
fn hostile_byte_strings_bind_only_what_the_rules_allow() {
    let mut rng = Rng::seeded(0x5eed_0002);
    for round in 0..ROUNDS {
        check_config(&hostile_bytes(&mut rng), round);
    }
}
