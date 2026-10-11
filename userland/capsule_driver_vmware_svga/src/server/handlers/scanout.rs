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

//! OP_SET_SCANOUT: the primary resource on scanout 0. The mode is already
//! set and scanning out, so this only checks the request is that one; a
//! resource of 0 (scanout off) is taken and the last picture stays.

use super::rect::rect_at;
use crate::constants::mode::PRIMARY_RESOURCE;
use crate::protocol::{le_u32, E_INVAL};
use crate::setup::Driver;

const BODY_LEN: usize = 24;

pub fn scanout(d: &mut Driver, body: &[u8]) -> i32 {
    if body.len() != BODY_LEN || le_u32(body, 0) != Some(0) {
        return E_INVAL;
    }
    match le_u32(body, 4) {
        Some(0) => 0,
        Some(PRIMARY_RESOURCE) if rect_at(d, body, 8).is_some() => 0,
        _ => E_INVAL,
    }
}
