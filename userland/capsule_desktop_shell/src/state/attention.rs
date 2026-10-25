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

//! Ctrl+Alt+Delete, the secure attention chord. The input router hands it
//! to the shell alone, over any grab or prompt, so whatever a window draws,
//! the chord brings up something only the system can show: its own prompt,
//! from which the screen is hidden, and the way back to it.

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Attention {
    #[default]
    None,
    /// The system's prompt: hide the screen, or go back.
    Prompt,
    /// The screen hidden: everything covered until the chord again. It is
    /// not a lock: with no accounts, anyone at the keyboard can show it.
    Hidden,
}

impl Attention {
    /// The chord pressed. From the hidden screen it leads back to the desktop: no
    /// window can send it, so a person at the keyboard is asking. There are
    /// no accounts to check a password against yet.
    pub fn chord(self) -> Attention {
        match self {
            Attention::Hidden => Attention::None,
            _ => Attention::Prompt,
        }
    }

    /// The prompt answered: hide the screen, or back to the desktop.
    pub fn answer(self, hide: bool) -> Attention {
        match self {
            Attention::Prompt if hide => Attention::Hidden,
            Attention::Prompt => Attention::None,
            other => other,
        }
    }

    pub fn prompt(self) -> bool {
        self == Attention::Prompt
    }

    pub fn hidden(self) -> bool {
        self == Attention::Hidden
    }
}
