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

//! One USB port's facts, and whether a user can reach it.

use super::pld::Pld;
use super::upc::{Upc, PROPRIETARY};
use crate::namespace::Id;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reach {
    /// Not connectable, proprietary, placed where no user sees it, or in
    /// the lid, which holds a built-in camera or microphone and no socket.
    Internal,
    /// Connectable to a socket a user can see.
    External,
    /// Firmware says too little to tell.
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsbPort {
    pub id: Id,
    /// The port of the hub above this port's hub, 0 for the root hub.
    pub parent: u8,
    pub port: u8,
    pub upc: Option<Upc>,
    pub pld: Option<Pld>,
}

impl UsbPort {
    pub fn reach(&self) -> Reach {
        match (self.upc, self.pld) {
            (Some(u), _) if !u.connectable || u.connector == PROPRIETARY => Reach::Internal,
            (_, Some(p)) if !p.user_visible || p.lid => Reach::Internal,
            (Some(_), Some(p)) if p.user_visible => Reach::External,
            _ => Reach::Unknown,
        }
    }
}
