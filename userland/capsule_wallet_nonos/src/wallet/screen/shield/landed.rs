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

//! The landed rows on Shield home (B7, v1.4.56): money found where it was
//! due, and the restore scan, which runs only from its own row and says
//! first that the RPC host may see the addresses it asks about.

use alloc::format;
use alloc::vec::Vec;

use nonos_app_skeleton::PaintBuffer;

use crate::wallet::etna::parts::fact::fact;
use crate::wallet::etna::parts::quiet::quiet_group;
use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::tokens::{GAP, TEXT_3};
use crate::wallet::etna::wrap::wrapped;
use crate::wallet::etna::Role;
use crate::wallet::order::fees::exact;
use crate::wallet::screen::hits::{self, Press};
use crate::wallet::shield::landed::line;
use crate::wallet::shield::scan::ScanState;
use crate::wallet::state::State;

pub const ROW_SCAN: u8 = 1;
pub const ROW_FURTHER: u8 = 2;
const NOTE: &str = "A restore scan asks the network about this wallet's unused addresses \
     one at a time; the RPC host may see the addresses asked.";

pub fn paint(state: &State, fb: &mut PaintBuffer, c: Rect, y: u32) -> u32 {
    let l = &state.meter.landed;
    let mut h = 0;
    let due = l.due.entries.iter().filter_map(|e| e.nox.filter(|n| *n > 0).map(|n| (e.address, n)));
    let scanned = l.scan.found.iter().map(|f| (f.1, f.2));
    for (address, nox) in due.chain(scanned) {
        h += fact(fb, c.x, y + h, c.w, "landed", &line(&address, &exact(nox))) + GAP / 2;
    }
    let (said, row) = match l.scan.state {
        ScanState::Idle => (None, Some(("Look for refunds after a restore", ROW_SCAN))),
        ScanState::Deriving(_) | ScanState::Reading => {
            let at = l.scan.batch.get(l.scan.at).map_or(0, |b| b.0);
            (Some(format!("scanning address {at}, one at a time")), None)
        }
        ScanState::Stopped => (
            Some(format!("{} found so far", l.scan.found.len())),
            Some(("Scan further", ROW_FURTHER)),
        ),
    };
    let t = (y + h) as i32;
    h += wrapped(fb, c.x as i32, t, c.w as i32, Role::Lead, NOTE, TEXT_3) as u32 + GAP / 2;
    if let Some(s) = said {
        h += fact(fb, c.x, y + h, c.w, "scan", &s) + GAP / 2;
    }
    if let Some((title, id)) = row {
        let mut at = [Rect::default(); 1];
        h += quiet_group(fb, c.x, y + h, c.w, &Vec::from([(title, true)]), &mut at);
        hits::put(Press::Row(id), at[0]);
    }
    h
}
