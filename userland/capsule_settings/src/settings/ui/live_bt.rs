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

//! The Bluetooth page's live rows and its card's badge, worded from what
//! driver.bt_usb0 last said (`crate::bluetooth::panel`).

use crate::bluetooth::panel::{controller, scanning, status, Reach};
use crate::settings::schema::rows::{BtLive, Tone};
use crate::settings::state::State;

use super::live_bt_pair::{asked, pairing};
use super::valbuf::ValBuf;

pub fn bt(b: &mut ValBuf, state: &State, which: BtLive) -> Tone {
    let p = &state.bt;
    let (words, tone) = match which {
        BtLive::Controller => controller(p),
        BtLive::Status => status(p),
        BtLive::Scanning => scanning(p),
        BtLive::Window => return pairing(b, p),
        BtLive::Asked => return asked(b, p),
    };
    b.push_str(words);
    tone
}

/// The badge: scanning on or off, or why there is nothing to ask.
pub fn bt_pill(state: &State) -> (ValBuf, Tone) {
    let mut b = ValBuf::new();
    let tone = bt(&mut b, state, BtLive::Scanning);
    if state.bt.reach != Reach::Answered {
        b = ValBuf::new();
        b.push_str(if state.bt.reach == Reach::NoAnswer { "Not answering" } else { "Off" });
    }
    (b, tone)
}
