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

//! KEYS notices the window manager could not deliver yet. A window learns it
//! has the keyboard from one notice, sent when focus moves; an inbox that is
//! full at that moment (an owner not reading while it starts) used to lose
//! it for good, and the window never took a key. An owed notice is offered
//! again on every turn until it lands, the project is gone, or a newer notice
//! for the same window takes its place.

/// How many windows can be owed a notice at once. Focus moves one window to
/// another, so a turn owes at most two; this leaves room for owners that stay
/// full for many turns.
pub const OWED_MAX: usize = 16;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Owed {
    pub owner_pid: u32,
    pub window_id: u32,
    pub has: bool,
}

pub struct OwedKeys {
    slots: [Option<Owed>; OWED_MAX],
}

impl Default for OwedKeys {
    fn default() -> Self {
        Self::new()
    }
}

impl OwedKeys {
    pub const fn new() -> Self {
        Self { slots: [None; OWED_MAX] }
    }

    /// Owe `notice`. A notice already owed for the same window is replaced:
    /// only the latest word on its keyboard is true. With every slot taken,
    /// the oldest owed notice gives way.
    pub fn owe(&mut self, notice: Owed) {
        let same = |o: &Option<Owed>| {
            o.is_some_and(|o| o.owner_pid == notice.owner_pid && o.window_id == notice.window_id)
        };
        if let Some(slot) = self.slots.iter_mut().find(|o| same(o)) {
            *slot = Some(notice);
            return;
        }
        if let Some(slot) = self.slots.iter_mut().find(|o| o.is_none()) {
            *slot = Some(notice);
            return;
        }
        self.slots.rotate_left(1);
        self.slots[OWED_MAX - 1] = Some(notice);
    }

    /// Send `notice` now, `tries` times at most with `wait` between, while
    /// `full` says the project's inbox was full; still full after the last
    /// try, it is owed. Delivered, or refused for good, it settles what was
    /// owed that window.
    pub fn tell(
        &mut self,
        notice: Owed,
        tries: u32,
        mut full: impl FnMut(&Owed) -> bool,
        mut wait: impl FnMut(),
    ) {
        for attempt in 0..tries {
            if !full(&notice) {
                self.settled(notice.owner_pid, notice.window_id);
                return;
            }
            if attempt + 1 < tries {
                wait();
            }
        }
        self.owe(notice);
    }

    /// A notice for this window was delivered: nothing is owed it any more.
    pub fn settled(&mut self, owner_pid: u32, window_id: u32) {
        for slot in self.slots.iter_mut() {
            if slot.is_some_and(|o| o.owner_pid == owner_pid && o.window_id == window_id) {
                *slot = None;
            }
        }
    }

    /// Offer every owed notice once through `send`, which says whether the
    /// inbox was still full: delivered, or refused for good (the inbox is
    /// gone), the notice is settled; still full, it stays owed.
    pub fn offer_again(&mut self, mut still_full: impl FnMut(&Owed) -> bool) {
        for slot in self.slots.iter_mut() {
            let Some(owed) = *slot else { continue };
            if !still_full(&owed) {
                *slot = None;
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.slots.iter().all(Option::is_none)
    }
}
