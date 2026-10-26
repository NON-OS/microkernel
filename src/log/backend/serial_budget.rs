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

//! The serial echo's budget, pure: PER_SECOND lines a second for each tag
//! (a line's leading `[TAG]`; tags past SLOTS share the last slot), so a
//! refusal provoked in a loop cannot hold a CPU at 115200 baud. Lines over
//! it are counted and said by the tag's next echoed line; the RAM log keeps
//! all. Held in kernel_proofs (log_budget).

pub(super) const PER_SECOND: u32 = 8;
pub(super) const SLOTS: usize = 16;
#[derive(Clone, Copy)]
struct Slot {
    key: Option<u64>,
    second: u64,
    sent: u32,
    held: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Echo {
    Line,
    After(u32), // say first that this many were not echoed

    Not,
}
pub(super) struct Budget([Slot; SLOTS]);

/// A line's tag: its leading `[...]` of at most 24 bytes, or none.
pub(super) fn tag(msg: &[u8]) -> &[u8] {
    match msg.iter().take(24).position(|&b| b == b']') {
        Some(end) if msg.first() == Some(&b'[') => &msg[..=end],
        _ => b"",
    }
}

impl Budget {
    pub(super) const fn new() -> Self {
        Budget([Slot { key: None, second: 0, sent: 0, held: 0 }; SLOTS])
    }

    /// Whether the line `msg`, written at `now_ms`, is echoed.
    pub(super) fn admit(&mut self, msg: &[u8], now_ms: u64) -> Echo {
        // A shared key only shares a budget.
        let k = Some(tag(msg).iter().fold(0u64, |h, &b| h.rotate_left(5) ^ u64::from(b)));
        let own = self.0.iter().position(|s| s.key == k);
        let at = own.or_else(|| self.0.iter().position(|s| s.key.is_none())).unwrap_or(SLOTS - 1);
        let s = &mut self.0[at];
        let second = now_ms / 1000;
        if s.key.is_none() || s.second != second {
            let held = core::mem::take(&mut s.held);
            *s = Slot { key: s.key.or(k), second, sent: 1, held: 0 };
            return if held > 0 { Echo::After(held) } else { Echo::Line };
        }
        if s.sent < PER_SECOND {
            s.sent += 1;
            return Echo::Line;
        }
        s.held = s.held.saturating_add(1);
        Echo::Not
    }
}
