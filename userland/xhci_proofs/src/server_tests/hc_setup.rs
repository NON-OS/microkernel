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

//! The setup packet a control transfer opens with (USB 2.0 section 9.3).

use crate::trb::Trb;

/// The setup packet, from the Setup Stage TRB's immediate data.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Setup {
    pub request_type: u8,
    pub request: u8,
    pub value: u16,
    pub index: u16,
    pub length: u16,
}

impl Setup {
    pub(super) fn of(trb: &Trb) -> Self {
        let (d0, d1) = (trb.d0, trb.d1);
        let (request_type, request) = (d0 as u8, (d0 >> 8) as u8);
        Self {
            request_type,
            request,
            value: (d0 >> 16) as u16,
            index: d1 as u16,
            length: (d1 >> 16) as u16,
        }
    }
}
