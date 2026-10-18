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

//! A cut or malformed Transition Disable KDE is ignored without a panic.

use super::td_sup_tests::handshake;
use nonos_wifi_core::eapol::kde::{parse_key_data, TD_WPA3_PERSONAL};
use nonos_wifi_core::wpa::supplicant::State;

pub(super) const TD: [u8; 7] = [0xDD, 5, 0x50, 0x6F, 0x9A, 0x20, TD_WPA3_PERSONAL];

#[test]
fn a_cut_or_malformed_transition_disable_kde_is_ignored_without_a_panic() {
    let cut = handshake(&[0xDD, 4, 0x50, 0x6F, 0x9A, 0x20]);
    assert_eq!((cut.state(), cut.transition_disable()), (State::Connected, None), "no bitmap");
    let other_oui = handshake(&[0xDD, 5, 0x00, 0x50, 0xF2, 0x20, 1]);
    assert_eq!(other_oui.transition_disable(), None, "another OUI's type 0x20");
    let twice = [TD, [0xDD, 5, 0x50, 0x6F, 0x9A, 0x20, 0]].concat();
    assert_eq!(parse_key_data(&twice).unwrap().transition_disable, Some(1), "the first is kept");
    assert!(parse_key_data(&[0xDD, 9, 0x50, 0x6F, 0x9A, 0x20, 1]).is_none(), "runs past the end");
    for n in 0..=TD.len() {
        let td = parse_key_data(&TD[..n]).and_then(|k| k.transition_disable);
        assert_eq!(td, (n == TD.len()).then_some(1), "cut at {n}");
    }
}
