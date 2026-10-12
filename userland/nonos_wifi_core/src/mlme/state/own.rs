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

//! The MLME's own helpers: its secrets, its sequence and its failure.

use super::super::failure::MlmeFailure;
use super::kinds::{MlmeOutput, MlmeState};
use super::Mlme;

impl Mlme {
    pub(in crate::mlme) fn ssid(&self) -> &[u8] {
        &self.ssid[..self.ssid_len]
    }

    pub(in crate::mlme) fn secret(&self) -> &[u8] {
        &self.secret[..self.secret_len]
    }

    // Each transmitted management frame advances the sequence counter.
    pub(in crate::mlme) fn next_seq(&mut self) -> u16 {
        let s = self.seq;
        self.seq = self.seq.wrapping_add(1);
        s
    }

    pub(in crate::mlme) fn fail(&mut self, why: MlmeFailure) -> MlmeOutput {
        self.state = MlmeState::Failed;
        self.failure = Some(why);
        MlmeOutput::none()
    }
}
