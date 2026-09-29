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

//! Where the last painted screen put the things that can be pressed. The
//! same pass that draws them writes them here, so a click is tested against
//! exactly what the person saw.

use spin::Mutex;

use crate::wallet::etna::rect::Rect;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Press {
    Send,
    Receive,
    Swap,
    Shield,
    Settings,
    Accounts,
    Back,
    Dismiss,
    Footer(u8),
}

const MAX: usize = 16;
static HITS: Mutex<([(Press, Rect); MAX], usize)> =
    Mutex::new(([(Press::Back, Rect::new(0, 0, 0, 0)); MAX], 0));

pub fn clear() {
    HITS.lock().1 = 0;
}

pub fn put(what: Press, at: Rect) {
    let mut h = HITS.lock();
    let n = h.1;
    if n < MAX {
        h.0[n] = (what, at);
        h.1 = n + 1;
    }
}

pub fn at(x: u32, y: u32) -> Option<Press> {
    let h = HITS.lock();
    h.0[..h.1].iter().find(|(_, r)| r.contains(x, y)).map(|(p, _)| *p)
}

static LIMIT: Mutex<u32> = Mutex::new(0);

/// How far the screen on show may scroll: where its content ended, less the
/// room above the fixed foot. Written by the screen as it paints.
pub fn reach(content_end: u32, scroll: u32, content_bottom: u32) {
    *LIMIT.lock() = (content_end + scroll).saturating_sub(content_bottom);
}

pub fn limit() -> u32 {
    *LIMIT.lock()
}
