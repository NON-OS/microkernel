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

//! What a capabilities read should come to, read off the sink's log by the
//! test's own rule: each request names the string's length so far; a
//! request refused ends it; a reply taken whole that frames a fragment
//! adds it, or ends it at the string's end, or refuses it past CAPS_MAX;
//! three misses in a row refuse it.

use super::ci_bus::taken;
use super::ci_own::{fragment, request};
use super::sink::Try;
use crate::ddc::caps::Caps;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum End {
    Got,
    Unparsed,
    Absent,
    Over,
    Missed,
    Open,
}

pub fn judge_caps(log: &[Try]) -> End {
    let (mut text, mut misses, mut i) = (Vec::new(), 0, 0);
    let end = loop {
        let Some(tx) = log.get(i) else { break End::Open };
        let asked = request(&[0xF3, (text.len() >> 8) as u8, text.len() as u8]);
        assert!(asked.starts_with(&tx.sent) && (tx.nak || tx.sent == asked), "request");
        if tx.nak {
            break End::Absent;
        }
        let Some(rx) = log.get(i + 1) else { break End::Open };
        i += 2;
        assert!(rx.got.len() <= 40, "a word past the 38 bytes asked");
        match taken(rx, 38).then(|| fragment(text.len() as u16, &rx.got[..38])).flatten() {
            Some(d) if d.is_empty() && Caps::parse(&text).is_some() => break End::Got,
            Some(d) if d.is_empty() => break End::Unparsed,
            Some(d) if text.len() + d.len() > 1024 => break End::Over,
            Some(d) => (text, misses) = ([text, d].concat(), 0),
            None if misses + 1 >= 3 => break End::Missed,
            None => misses += 1,
        }
    };
    assert!(
        end == End::Open || i + (end == End::Absent) as usize == log.len(),
        "a message past the end"
    );
    end
}
