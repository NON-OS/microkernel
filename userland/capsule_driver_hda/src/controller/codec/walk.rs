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

//! Reading a codec's audio function group into a `Codec`.
//!
//! Every count here comes from the codec, so every loop is capped: a part
//! whose firmware did not load answers each parameter with the same word,
//! and an unbounded walk over 255 claimed nodes is a boot that seems to stop.

use alloc::boxed::Box;

use super::walk_afg::{find_afg, param, subnodes};
use super::walk_widget::widget;
use super::widget::{Codec, MAX_WIDGETS};
use crate::constants::{
    PARAM_AMP_IN_CAP, PARAM_AMP_OUT_CAP, PARAM_GPIO_CAP, PARAM_PCM, PARAM_VENDOR_ID,
    VERB_GET_SUBSYSTEM_ID,
};
use crate::controller::compose_verb;
use crate::controller::verb::Link;
use crate::error::HdaResult;

pub fn walk(link: &mut Link, cad: u8) -> HdaResult<Option<Box<Codec>>> {
    let mut c = Box::new(Codec::new(cad));
    c.vendor_id = param(link, cad, 0, PARAM_VENDOR_ID)?;
    let Some(afg) = find_afg(link, cad)? else { return Ok(None) };
    c.afg = afg;
    c.subsystem_id = link.send(compose_verb(cad, afg, VERB_GET_SUBSYSTEM_ID, 0))?;
    c.gpio_count = (param(link, cad, afg, PARAM_GPIO_CAP)? & 0xff) as u8;
    let afg_in = param(link, cad, afg, PARAM_AMP_IN_CAP)?;
    let afg_out = param(link, cad, afg, PARAM_AMP_OUT_CAP)?;
    let afg_pcm = param(link, cad, afg, PARAM_PCM)?;
    let (start, count) = subnodes(link, cad, afg)?;
    let mut i = 0u8;
    while i < count && (i as usize) < MAX_WIDGETS {
        let nid = start.wrapping_add(i);
        if nid > 0x7f {
            break;
        }
        let w = widget(link, cad, nid, (afg_in, afg_out, afg_pcm))?;
        c.push(w);
        i += 1;
    }
    Ok(Some(c))
}
