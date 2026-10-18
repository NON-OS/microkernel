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

//! The parameter read every step of a codec walk makes, and the audio
//! function group found under the root node with it.

use crate::constants::{
    FUNCTION_GROUP_AUDIO, PARAM_FUNCTION_GROUP_TYPE, PARAM_SUBNODE_COUNT, VERB_GET_PARAMETER,
};
use crate::controller::compose_verb;
use crate::controller::verb::Link;
use crate::error::HdaResult;

/// Function groups looked at under the root node: a codec has one audio
/// group and at most a modem group beside it.
pub const FG_CAP: u8 = 8;

pub fn param(link: &mut Link, cad: u8, nid: u8, p: u16) -> HdaResult<u32> {
    link.send(compose_verb(cad, nid, VERB_GET_PARAMETER, p))
}

pub(super) fn subnodes(link: &mut Link, cad: u8, nid: u8) -> HdaResult<(u8, u8)> {
    let r = param(link, cad, nid, PARAM_SUBNODE_COUNT)?;
    Ok((((r >> 16) & 0xff) as u8, (r & 0xff) as u8))
}

/// The audio function group of codec `cad`, or none for a codec without one
/// (a modem).
pub fn find_afg(link: &mut Link, cad: u8) -> HdaResult<Option<u8>> {
    let (start, count) = subnodes(link, cad, 0)?;
    let mut i = 0u8;
    while i < count && i < FG_CAP {
        let nid = start.wrapping_add(i);
        if param(link, cad, nid, PARAM_FUNCTION_GROUP_TYPE)? & 0xff == FUNCTION_GROUP_AUDIO {
            return Ok(Some(nid));
        }
        i += 1;
    }
    Ok(None)
}
