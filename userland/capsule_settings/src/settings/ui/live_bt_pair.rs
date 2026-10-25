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

//! The pairing rows: the window, and what pairing asks of the user.

use nonos_bt_proto::{Asked, Link};
use nonos_libc::mk_uptime_ms;

use crate::bluetooth::panel::{window, Panel, Reach, DIGITS};
use crate::settings::schema::rows::Tone;

use super::valbuf::ValBuf;

pub(super) fn pairing(b: &mut ValBuf, p: &Panel) -> Tone {
    match window(p, mk_uptime_ms()) {
        None if p.reach == Reach::Answered => b.push_str("Only paired devices connect"),
        None => b.push_str("--"),
        Some(left) => {
            b.push_str("A new device may pair");
            if let Some(s) = left {
                b.push_str(", ");
                b.push_u32(s as u32);
                b.push_str(" s left");
            }
            return Tone::Ok;
        }
    }
    Tone::Idle
}

pub(super) fn asked(b: &mut ValBuf, p: &Panel) -> Tone {
    let waiting = p.has(Link::WAITING);
    match p.asked {
        Asked::None if waiting => b.push_str("Waiting"),
        Asked::None => {
            b.push_str(if p.reach == Reach::Answered { "Nothing" } else { "--" });
            return Tone::Idle;
        }
        Asked::Show(n) => six(b, "Type ", n, " on it, then its Enter"),
        Asked::Confirm(n) => six(b, "Does it show ", n, "? Y or N"),
        Asked::Enter => {
            b.push_str("Its digits: ");
            b.push_bytes(p.digits.as_bytes());
            (p.digits.as_bytes().len()..DIGITS).for_each(|_| b.push(b'_'));
        }
    }
    Tone::Warn
}

/// A passkey as six digits, leading zeros kept (Vol 3, Part H, 2.3.5.3).
fn six(b: &mut ValBuf, before: &str, n: u32, after: &str) {
    b.push_str(before);
    (0..6).rev().for_each(|i| b.push(b'0' + (n / 10u32.pow(i) % 10) as u8));
    b.push_str(after);
}
