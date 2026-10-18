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

//! What a key asks of the service, decided without asking it, so the
//! proofs drive every key. A key asks only what fits what the service last
//! said: Y answers only a number to confirm, Enter only six digits typed.

use nonos_bt_proto::ops::{OP_FORGET, OP_PAIR, OP_SCAN};
use nonos_bt_proto::{forget_body, scan_body, Link};

use super::answer::answer;
use super::key::BtKey;
use super::known::Panel;
use super::notice::unreached;
use super::reach::Reach;
use super::request::{Request, Step};

pub fn act(p: &mut Panel, key: BtKey) -> Step {
    match key {
        BtKey::Close => return Step::Close,
        BtKey::NextSection => return Step::NextSection,
        BtKey::PrevSection => return Step::PrevSection,
        BtKey::Up => p.cursor = p.cursor.saturating_sub(1),
        BtKey::Down if p.cursor + 1 < p.bond_count => p.cursor += 1,
        BtKey::Down => {}
        _ if p.reach != Reach::Answered => p.notice = Some(unreached(p.reach)),
        BtKey::Scan => return ask(OP_SCAN, &scan_body(!p.has(Link::SCANNING))),
        BtKey::Pair => return ask(OP_PAIR, &[]),
        BtKey::Forget if p.cursor < p.bond_count => {
            let b = p.bonds[p.cursor];
            return ask(OP_FORGET, &forget_body(b.kind, &b.addr));
        }
        BtKey::Forget => p.notice = Some("No device is paired"),
        key => return answer(p, key),
    }
    Step::Shown
}

pub(super) fn ask(op: u16, body: &[u8]) -> Step {
    Step::Ask(Request::new(op, body))
}
