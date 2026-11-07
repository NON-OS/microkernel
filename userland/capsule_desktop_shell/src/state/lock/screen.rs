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

//! The lock screen: open, locked with a field, or waiting on the keyring's
//! answer. Wrong entries show their count, and the wait the keyring holds
//! the next attempt for is counted down on screen.

use super::field::Field;
use super::keys::{edit, Edit};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Phase {
    /// Unlocked: nothing is covered.
    #[default]
    Open,
    Locked,
    /// The passphrase went to the keyring and its answer is awaited.
    Checking,
}

/// The keyring's answer to a check.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Checked {
    Right,
    Wrong {
        failures: u32,
        wait_ms: u32,
    },
    Wait {
        wait_ms: u32,
    },
    /// No answer, or one that cannot be acted on: the screen stays locked.
    Unavailable,
}

/// What the screen says under the field.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Note {
    #[default]
    None,
    Wrong {
        failures: u32,
    },
    Unavailable,
}

#[derive(Default)]
pub struct LockScreen {
    phase: Phase,
    pub field: Field,
    note: Note,
    open_at_ms: i64,
}

/// What a key on the lock screen asks for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Asked {
    Nothing,
    Redraw,
    /// Send the field to the keyring.
    Check,
}

impl LockScreen {
    pub fn phase(&self) -> Phase {
        self.phase
    }

    /// Whether the screen is covered.
    pub fn covers(&self) -> bool {
        self.phase != Phase::Open
    }

    pub fn note(&self) -> Note {
        self.note
    }

    /// Lock: an empty field and nothing said.
    pub fn lock(&mut self) {
        if self.phase == Phase::Open {
            self.field.clear();
            self.note = Note::None;
        }
        self.phase = Phase::Locked;
    }

    /// Milliseconds before the keyring takes another attempt.
    pub fn wait_left(&self, now_ms: i64) -> u64 {
        u64::try_from(self.open_at_ms.saturating_sub(now_ms)).unwrap_or(0)
    }

    /// A key while locked.
    pub fn key(&mut self, code: u32, now_ms: i64) -> Asked {
        if self.phase != Phase::Locked {
            return Asked::Nothing;
        }
        match edit(&mut self.field, code) {
            Edit::Typed => Asked::Redraw,
            Edit::Escape => {
                self.field.clear();
                Asked::Redraw
            }
            Edit::Enter if self.field.is_empty() || self.wait_left(now_ms) > 0 => Asked::Nothing,
            Edit::Enter => {
                self.phase = Phase::Checking;
                Asked::Check
            }
            Edit::Ignored => Asked::Nothing,
        }
    }

    /// The keyring's answer. The field is wiped whatever it said.
    pub fn answered(&mut self, checked: Checked, now_ms: i64) {
        self.field.clear();
        match checked {
            Checked::Right => {
                self.phase = Phase::Open;
                self.note = Note::None;
                self.open_at_ms = 0;
            }
            Checked::Wrong { failures, wait_ms } => {
                self.phase = Phase::Locked;
                self.note = Note::Wrong { failures };
                self.open_at_ms = now_ms.saturating_add(i64::from(wait_ms));
            }
            Checked::Wait { wait_ms } => {
                self.phase = Phase::Locked;
                self.open_at_ms = now_ms.saturating_add(i64::from(wait_ms));
            }
            Checked::Unavailable => {
                self.phase = Phase::Locked;
                self.note = Note::Unavailable;
            }
        }
    }

    /// The wait the keyring holds the next attempt for, as read from its
    /// standing at start (a count kept across a restart).
    pub fn hold_for(&mut self, wait_ms: u32, now_ms: i64) {
        self.open_at_ms = now_ms.saturating_add(i64::from(wait_ms));
    }
}
