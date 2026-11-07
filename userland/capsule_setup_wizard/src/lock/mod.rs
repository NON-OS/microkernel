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

//! The lock screen's passphrase as setup asks for it: typed, typed again,
//! and held only until the review sends it to the keyring, which keeps a
//! slow hash of it. Both copies are wiped once sent and before setup exits.

use nonos_lock::field::Field;
use nonos_lock::rules::{acceptable, chosen, Refusal};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    New,
    Again,
    /// Typed twice and the same: sent at the review.
    Chosen,
}

/// What a key on the step asks for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Keyed {
    Stay,
    Advance,
    Back,
}

pub struct LockStep {
    pub new: Field,
    pub again: Field,
    pub stage: Stage,
    pub refused: Option<Refusal>,
    /// The keyring did not take it at the review.
    pub failed: bool,
}

const BACKSPACE: u32 = 0x08;
const ENTER: u32 = 0x0D;
const ENTER_LF: u32 = 0x0A;
const ESC: u32 = 0x1B;

impl LockStep {
    pub fn new() -> Self {
        LockStep {
            new: Field::default(),
            again: Field::default(),
            stage: Stage::New,
            refused: None,
            failed: false,
        }
    }

    /// The passphrase to send, once chosen.
    pub fn chosen(&self) -> Option<&[u8]> {
        (self.stage == Stage::Chosen).then(|| self.new.bytes())
    }

    /// Wipe both copies; the step starts again if it is shown again.
    pub fn forget(&mut self) {
        self.new.clear();
        self.again.clear();
        self.stage = Stage::New;
    }

    pub fn key(&mut self, code: u32) -> Keyed {
        match (self.stage, code) {
            (Stage::Chosen, ENTER | ENTER_LF) => Keyed::Advance,
            (Stage::Chosen, ESC) => Keyed::Back,
            (Stage::Chosen, BACKSPACE) => {
                self.forget();
                Keyed::Stay
            }
            (Stage::Chosen, _) => Keyed::Stay,
            (Stage::New, ESC) => Keyed::Back,
            (Stage::Again, ESC) => {
                self.again.clear();
                self.stage = Stage::New;
                Keyed::Stay
            }
            (_, ENTER | ENTER_LF) => {
                self.enter();
                if self.stage == Stage::Chosen {
                    Keyed::Advance
                } else {
                    Keyed::Stay
                }
            }
            (stage, c) => {
                let field = if stage == Stage::New { &mut self.new } else { &mut self.again };
                if c == BACKSPACE {
                    field.pop();
                } else if let Some(ch) = char::from_u32(c).filter(|ch| !ch.is_control()) {
                    let _ = field.push(ch);
                }
                self.refused = None;
                Keyed::Stay
            }
        }
    }

    fn enter(&mut self) {
        match self.stage {
            Stage::New => match acceptable(self.new.bytes()) {
                Ok(()) => {
                    self.stage = Stage::Again;
                    self.refused = None;
                }
                Err(r) => self.refused = Some(r),
            },
            Stage::Again => match chosen(self.new.bytes(), self.again.bytes()) {
                Ok(()) => {
                    self.stage = Stage::Chosen;
                    self.refused = None;
                }
                Err(r) => {
                    self.forget();
                    self.refused = Some(r);
                }
            },
            Stage::Chosen => {}
        }
    }
}

impl Default for LockStep {
    fn default() -> Self {
        Self::new()
    }
}
