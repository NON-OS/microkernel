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

//! One PCI interrupt route.

use crate::namespace::Id;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Route {
    /// The device number on the bridge's bus (the high word of the address).
    pub slot: u16,
    /// 0 to 3, INTA to INTD.
    pub pin: u8,
    /// The link device, when the entry names one.
    pub link: Option<Id>,
    /// None when a link's current resources name no interrupt.
    pub gsi: Option<u32>,
}
