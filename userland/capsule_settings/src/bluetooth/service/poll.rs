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

//! What the page asks each time it looks: LINK and PASSKEY every time, so
//! a question pairing asks shows at once; HEALTH, BONDS and BATTERY when
//! the page opens, after a change it asked for, when LINK moved, and every
//! `LIST_MS` besides, since a battery level changes with nothing else.

use nonos_bt_proto::ops::{E_OK, OP_BATTERY, OP_BONDS, OP_HEALTH, OP_LINK, OP_PASSKEY};
use nonos_bt_proto::{DATA_AT, REPLY_MAX};
use nonos_libc::mk_uptime_ms;

use super::call::call;
use crate::bluetooth::panel::{refused, Panel, Reach};

const LIST_MS: i64 = 5000;

type Take = fn(&mut Panel, &[u8]) -> bool;

pub fn poll(p: &mut Panel, full: bool) {
    let now = mk_uptime_ms();
    let before = p.link;
    let mut rx = [0u8; REPLY_MAX];
    if !ask(p, OP_LINK, &mut rx, Panel::took_link)
        || !ask(p, OP_PASSKEY, &mut rx, Panel::took_asked)
    {
        return;
    }
    let stale = p.listed_ms.is_none_or(|t| now.saturating_sub(t) >= LIST_MS);
    if !(full || stale || before != p.link) {
        return;
    }
    p.listed_ms = Some(now);
    let _ = ask(p, OP_HEALTH, &mut rx, Panel::took_health)
        && ask(p, OP_BONDS, &mut rx, Panel::took_bonds)
        && ask(p, OP_BATTERY, &mut rx, Panel::took_levels);
}

/// One reply taken. A refusal is said on the page; a reply the panel cannot
/// read, or none, clears what the service said before.
fn ask(p: &mut Panel, op: u16, rx: &mut [u8; REPLY_MAX], take: Take) -> bool {
    let reach = match call(p, op, &[], rx) {
        Ok((E_OK, end)) if take(p, &rx[DATA_AT..end]) => {
            p.reach = Reach::Answered;
            return true;
        }
        Ok((E_OK, _)) => Reach::NoAnswer,
        Ok((status, _)) => {
            p.notice = Some(refused(status));
            return false;
        }
        Err(reach) => reach,
    };
    p.lost(reach);
    false
}
