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

use super::tag::{ATTEST, DELIVERY, DEVICE, ISOLATION, MEASUREMENTS, POLICY, PUBLICS, RECORD};

/// What a container proves. The number is the anchor's kind byte and the
/// kind NoxProofAnchor records.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Isolation = 1,
    Delivery = 2,
    Authenticity = 3,
}

impl Kind {
    pub fn from_u16(v: u16) -> Option<Kind> {
        match v {
            1 => Some(Kind::Isolation),
            2 => Some(Kind::Delivery),
            3 => Some(Kind::Authenticity),
            _ => None,
        }
    }

    pub fn code(self) -> u8 {
        self as u8
    }

    /// The exact sections the kind carries, ascending. Nothing missing and
    /// nothing more, so one proof has one container and one hash.
    pub fn sections(self) -> &'static [u16] {
        match self {
            Kind::Isolation => &[RECORD, DEVICE, ISOLATION, PUBLICS, MEASUREMENTS, POLICY],
            Kind::Delivery => &[RECORD, DEVICE, DELIVERY, MEASUREMENTS],
            Kind::Authenticity => &[ATTEST],
        }
    }
}
