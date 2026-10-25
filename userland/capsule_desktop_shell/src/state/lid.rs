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

//! The lid, as the kernel's keys say it moved (KEY_LID_CLOSED and
//! KEY_LID_OPENED, its first reading among them), and what the shell asks
//! of the compositor for it. Pure, so the rule is proven on the host
//! (desktop_proofs lid_tests).
//!
//! Closing darkens the screen and nothing more: closing to lock, or to power
//! off, are the owner's policy rows, and slot in as choices of `OnClose`.
//! A compositor that was not told what the lid is, a restarted one among
//! them, is told again, so a shut lid never lights up behind it.

/// What closing the lid does.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OnClose {
    /// The compositor darkens every output; opening wakes them.
    Blank,
}

/// The choice until the owner's rows exist.
pub const ON_CLOSE: OnClose = OnClose::Blank;

/// Whether the screen is to be dark for a lid `closed` under `on_close`.
pub fn dark(closed: bool, on_close: OnClose) -> bool {
    match on_close {
        OnClose::Blank => closed,
    }
}

/// The darkness last asked for, and which compositor answered for what.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LidTold {
    dark: Option<bool>,
    told: Option<(bool, u32)>,
}

impl LidTold {
    /// A lid key: the darkness it asks for, kept until a compositor has it.
    pub fn heard(&mut self, closed: bool, on_close: OnClose) -> bool {
        let d = dark(closed, on_close);
        self.dark = Some(d);
        d
    }

    /// What to tell compositor `pid`, if it does not yet have it. Nothing
    /// before the first lid key: a machine with no lid is never told one.
    pub fn due(&self, pid: u32) -> Option<bool> {
        let d = self.dark?;
        (self.told != Some((d, pid))).then_some(d)
    }

    /// Compositor `pid` answered for `dark`.
    pub fn told(&mut self, dark: bool, pid: u32) {
        self.told = Some((dark, pid));
    }
}
