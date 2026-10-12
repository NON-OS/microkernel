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

//! What a driver reads of a handshake and the keys it gave.

use super::kinds::{Failure, State};
use super::Supplicant;
use crate::wpa::akm::Akm;

impl Supplicant {
    pub fn state(&self) -> State {
        self.state
    }

    /// Why the handshake failed, once `Failed`.
    pub fn failure(&self) -> Option<Failure> {
        self.failure
    }

    /// The AKM this handshake runs.
    pub fn akm(&self) -> Akm {
        self.akm
    }

    /// Whether management frame protection was negotiated for this link.
    pub fn pmf(&self) -> bool {
        self.pmf
    }

    /// The pairwise temporal key (CCMP encryption key), valid once Connected.
    pub fn tk(&self) -> &[u8] {
        &self.ptk[32..48]
    }

    /// The group temporal key, valid once Connected.
    pub fn gtk(&self) -> &[u8] {
        &self.gtk[..self.gtk_len]
    }

    /// The group key's index (1-3), valid once Connected. Group-addressed
    /// frames name their key by it, and an AP moves it between 1 and 2 on
    /// every rekey, so it has to be installed where the AP says.
    pub fn gtk_id(&self) -> u8 {
        self.gtk_id
    }

    /// The packet number the AP last sent under the group key, from the Key
    /// RSC of the message that delivered it: group frames must exceed it.
    pub fn gtk_rsc(&self) -> u64 {
        self.gtk_rsc
    }

    /// The integrity group key, its index and the AP's IGTK packet number,
    /// when management frame protection delivered one.
    pub fn igtk(&self) -> Option<(&[u8; 16], u16, [u8; 6])> {
        self.igtk_set.then_some((&self.igtk, self.igtk_id, self.igtk_ipn))
    }
}
