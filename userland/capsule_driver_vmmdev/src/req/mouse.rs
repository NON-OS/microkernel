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

//! Mouse integration: switched on by telling the host the guest takes
//! absolute positions and reads them on an event; read on each event.

use super::page::Page;
use crate::constants::req::{GET_MOUSE_STATUS, SET_MOUSE_STATUS};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MouseStatus {
    pub features: u32,
    pub x: i32,
    pub y: i32,
}

pub fn set_mouse(p: &mut Page, features: u32) -> Result<(), i32> {
    p.perform(SET_MOUSE_STATUS, &[features, 0, 0]).map(drop)
}

pub fn get_mouse(p: &mut Page) -> Result<MouseStatus, i32> {
    let b = p.perform(GET_MOUSE_STATUS, &[0, 0, 0])?;
    Ok(MouseStatus { features: b[0], x: b[1] as i32, y: b[2] as i32 })
}
