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

//! Nothing is asked of a service that is not there, and the keys that
//! leave the page still leave it.

use crate::bluetooth::panel::key::BtKey;
use crate::bluetooth::panel::{act, Panel, Reach, Step};

#[test]
fn nothing_is_asked_of_a_service_that_did_not_answer() {
    for (reach, words) in [
        (Reach::Unasked, "Bluetooth is not running"),
        (Reach::NoService, "Bluetooth is not running"),
        (Reach::NoAnswer, "Bluetooth is not answering"),
    ] {
        let mut p = Panel::new();
        p.reach = reach;
        for key in [BtKey::Scan, BtKey::Pair, BtKey::Forget, BtKey::Yes, BtKey::Digit(b'1')] {
            assert_eq!(act(&mut p, key), Step::Shown, "{key:?}");
            assert_eq!(p.notice, Some(words));
        }
    }
    let mut p = Panel::new();
    assert_eq!(act(&mut p, BtKey::Close), Step::Close);
    assert_eq!(act(&mut p, BtKey::NextSection), Step::NextSection);
    assert_eq!(act(&mut p, BtKey::PrevSection), Step::PrevSection);
}
