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

//! The keys that type or answer a passkey, which do something only while
//! pairing asks for it (Vol 3, Part H, 2.3.5.1): digits and Enter for the
//! device's passkey, Y and N for a number both sides show.

use nonos_bt_proto::ops::OP_ANSWER;
use nonos_bt_proto::{answer_body, Asked, Reply};

use super::act::ask;
use super::key::BtKey;
use super::known::Panel;
use super::request::Step;

pub(super) fn answer(p: &mut Panel, key: BtKey) -> Step {
    let reply = match (key, p.asked) {
        (BtKey::Digit(d), Asked::Enter) => return shown(p.digits.push(d)),
        (BtKey::Erase, Asked::Enter) => {
            p.digits.pop();
            return Step::Shown;
        }
        (BtKey::Enter, Asked::Enter) => match p.digits.value() {
            Some(v) => Reply::Digits(v),
            None => {
                p.notice = Some("Type all six digits the device shows");
                return Step::Shown;
            }
        },
        (BtKey::Yes, Asked::Confirm(_)) => Reply::Same(true),
        (BtKey::No, Asked::Confirm(_)) => Reply::Same(false),
        (BtKey::No, Asked::Enter) => Reply::Decline,
        _ => return Step::Nothing,
    };
    answer_body(reply).map_or(Step::Nothing, |b| ask(OP_ANSWER, &b))
}

fn shown(changed: bool) -> Step {
    if changed {
        Step::Shown
    } else {
        Step::Nothing
    }
}
