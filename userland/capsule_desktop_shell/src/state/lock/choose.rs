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

//! The prompt that chooses the passphrase (none set) or changes it (the
//! current one first). Enter moves on; the new one is typed twice and must
//! meet `nonos_lock::rules` before it is sent.

use nonos_lock::rules::{acceptable, chosen, Refusal};

use super::field::Field;
use super::keys::{edit, Edit};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Purpose {
    /// None is set: the first one.
    First,
    /// Change the one set, which is asked for first.
    Change,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChooseStep {
    Current,
    New,
    Again,
    /// Sent to the keyring; its answer is awaited.
    Saving,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChooseNote {
    #[default]
    None,
    Refused(Refusal),
    /// The current passphrase was wrong.
    Wrong,
    /// Too soon after wrong entries.
    Wait,
    /// The keyring did not take it.
    Failed,
}

/// What a key in the prompt asks for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chosen {
    Nothing,
    Redraw,
    Cancel,
    /// Send current and new to the keyring.
    Save,
}

pub struct Choose {
    purpose: Purpose,
    step: ChooseStep,
    pub current: Field,
    pub new: Field,
    pub again: Field,
    note: ChooseNote,
}

impl Choose {
    pub fn new(purpose: Purpose) -> Choose {
        let step = match purpose {
            Purpose::First => ChooseStep::New,
            Purpose::Change => ChooseStep::Current,
        };
        Choose {
            purpose,
            step,
            current: Field::default(),
            new: Field::default(),
            again: Field::default(),
            note: ChooseNote::None,
        }
    }

    pub fn purpose(&self) -> Purpose {
        self.purpose
    }

    pub fn step(&self) -> ChooseStep {
        self.step
    }

    pub fn note(&self) -> ChooseNote {
        self.note
    }

    fn field(&mut self) -> Option<&mut Field> {
        match self.step {
            ChooseStep::Current => Some(&mut self.current),
            ChooseStep::New => Some(&mut self.new),
            ChooseStep::Again => Some(&mut self.again),
            ChooseStep::Saving => None,
        }
    }

    pub fn key(&mut self, code: u32) -> Chosen {
        let Some(field) = self.field() else {
            return Chosen::Nothing;
        };
        match edit(field, code) {
            Edit::Typed => Chosen::Redraw,
            Edit::Ignored => Chosen::Nothing,
            Edit::Escape => Chosen::Cancel,
            Edit::Enter => self.enter(),
        }
    }

    /// The Save button: Enter on the step that is up.
    pub fn enter(&mut self) -> Chosen {
        match self.step {
            ChooseStep::Current if self.current.is_empty() => Chosen::Nothing,
            ChooseStep::Current => {
                self.step = ChooseStep::New;
                self.note = ChooseNote::None;
                Chosen::Redraw
            }
            ChooseStep::New => {
                match acceptable(self.new.bytes()) {
                    Ok(()) => {
                        self.step = ChooseStep::Again;
                        self.note = ChooseNote::None;
                    }
                    Err(r) => self.note = ChooseNote::Refused(r),
                }
                Chosen::Redraw
            }
            ChooseStep::Again => match chosen(self.new.bytes(), self.again.bytes()) {
                Ok(()) => {
                    self.step = ChooseStep::Saving;
                    self.note = ChooseNote::None;
                    Chosen::Save
                }
                Err(r) => {
                    self.new.clear();
                    self.again.clear();
                    self.step = ChooseStep::New;
                    self.note = ChooseNote::Refused(r);
                    Chosen::Redraw
                }
            },
            ChooseStep::Saving => Chosen::Nothing,
        }
    }

    /// The keyring did not take it: the current one was wrong (or too soon
    /// after wrong ones), or it failed. Every field is wiped and the prompt
    /// starts again from its first step.
    pub fn refused(&mut self, note: ChooseNote) {
        self.current.clear();
        self.new.clear();
        self.again.clear();
        self.note = note;
        self.step = match self.purpose {
            Purpose::Change => ChooseStep::Current,
            Purpose::First => ChooseStep::New,
        };
    }
}
