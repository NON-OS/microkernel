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

//! The access point's beacon, its keys, and its sequence number.

use super::*;
use nonos_wifi_core::wpa::akm::Akm;

impl Ap {
    pub fn beacon(&self) -> Vec<u8> {
        let mut els: Vec<&[u8]> = vec![&self.rsne];
        if let Some(x) = &self.rsnxe {
            els.push(x);
        }
        beacon(&els)
    }

    pub(super) fn akm(&self) -> Akm {
        match self.sec {
            Security::Wpa2 => Akm::Psk,
            Security::Sae { .. } => Akm::Sae,
        }
    }

    pub fn tk(&self) -> [u8; 16] {
        self.ptk[32..48].try_into().unwrap()
    }

    pub(super) fn next_seq(&mut self) -> u16 {
        self.seq = self.seq.wrapping_add(1) & 0x0FFF;
        self.seq
    }
}
