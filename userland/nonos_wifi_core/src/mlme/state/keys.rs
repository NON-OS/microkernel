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

//! The handshake and the keys it gave, once connected.

use super::*;

impl Mlme {
    /// The supplicant, once connected: the data path keeps answering the
    /// access point's group key handshakes with it. A copy, which wipes its
    /// keys when it goes, as this machine's does, seeded for its rekeys
    /// from the join's randomness.
    pub fn supplicant(&self) -> Option<Supplicant> {
        let mut s = self.supplicant.clone().filter(|_| self.state == MlmeState::Connected)?;
        s.reseed_rekey(self.rekey_seed);
        Some(s)
    }

    /// The AP accepted the (re)association: the handshake began, whatever
    /// became of it.
    pub fn accepted(&self) -> bool {
        self.supplicant.is_some()
    }

    /// The pairwise temporal key, valid once Connected.
    pub fn tk(&self) -> Option<&[u8]> {
        self.supplicant.as_ref().filter(|_| self.state == MlmeState::Connected).map(|s| s.tk())
    }

    /// The group temporal key, valid once Connected.
    pub fn gtk(&self) -> Option<&[u8]> {
        self.supplicant.as_ref().filter(|_| self.state == MlmeState::Connected).map(|s| s.gtk())
    }

    /// The group key's index, valid once Connected.
    pub fn gtk_id(&self) -> Option<u8> {
        self.supplicant.as_ref().filter(|_| self.state == MlmeState::Connected).map(|s| s.gtk_id())
    }
}
