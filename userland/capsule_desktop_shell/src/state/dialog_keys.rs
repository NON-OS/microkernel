// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The keys a desktop dialog answers. Every dialog the shell draws (the live
//! session's installer offer, a Delete, a third-party launch, a package
//! install) has two buttons: one that acts, and one that leaves things as
//! they are. Tab and the arrows move the keyboard between them, Enter presses
//! the one it is on, and Esc is always the one that leaves things as they are.
//! A dialog opens with the keyboard on that safe button, so an Enter typed
//! before the dialog was read never deletes or installs anything.

pub const KEY_TAB: u32 = 0x09;
pub const KEY_ENTER: u32 = 0x0D;
pub const KEY_ESC: u32 = 0x1B;
pub const KEY_LEFT: u32 = 0x1203;
pub const KEY_RIGHT: u32 = 0x1204;

/// A dialog's two answers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Choice {
    /// The button that acts: Install NONOS, Delete, Approve.
    Act,
    /// The button that leaves things as they are: Not now, Cancel.
    Leave,
}

/// Which button the keyboard is on.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DialogFocus {
    on_act: bool,
}

impl DialogFocus {
    pub const fn new() -> Self {
        DialogFocus { on_act: false }
    }

    /// The button the keyboard is on, to draw its ring.
    pub fn focused(self) -> Choice {
        if self.on_act {
            Choice::Act
        } else {
            Choice::Leave
        }
    }

    /// What `code` does to an open dialog: an answer, or None when the key
    /// only moved the keyboard or means nothing here.
    pub fn key(&mut self, code: u32) -> Option<Choice> {
        match code {
            KEY_TAB | KEY_LEFT | KEY_RIGHT => {
                self.on_act = !self.on_act;
                None
            }
            KEY_ENTER => Some(self.focused()),
            KEY_ESC => Some(Choice::Leave),
            _ => None,
        }
    }

    /// Back to the safe button, once a dialog is answered, so the next one
    /// opens there.
    pub fn reset(&mut self) {
        self.on_act = false;
    }
}
