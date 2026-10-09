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

use super::conn::decode;
use super::pincfg::PinConfig;
use super::widget::{Codec, Widget, MAX_WIDGETS};
use crate::constants::{
    FUNCTION_GROUP_AUDIO, PARAM_AMP_IN_CAP, PARAM_AMP_OUT_CAP, PARAM_AUDIO_WIDGET_CAP,
    PARAM_CONNLIST_LEN, PARAM_FUNCTION_GROUP_TYPE, PARAM_GPIO_CAP, PARAM_PCM, PARAM_PIN_CAP,
    PARAM_SUBNODE_COUNT, PARAM_VENDOR_ID, VERB_GET_CONFIG_DEFAULT, VERB_GET_CONNECT_LIST,
    VERB_GET_PARAMETER, VERB_GET_SUBSYSTEM_ID, WCAP_AMP_OVRD, WCAP_CONN_LIST, WCAP_FORMAT_OVRD, WIDGET_TYPE_DAC,
    WIDGET_TYPE_PIN,
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

fn subnodes(link: &mut Link, cad: u8, nid: u8) -> HdaResult<(u8, u8)> {
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

fn widget(link: &mut Link, cad: u8, nid: u8, afg: (u32, u32, u32)) -> HdaResult<Widget> {
    let mut w = Widget::empty(nid);
    w.wcaps = param(link, cad, nid, PARAM_AUDIO_WIDGET_CAP)?;
    if w.wcaps & WCAP_AMP_OVRD != 0 {
        w.amp_in = if w.has_in_amp() { param(link, cad, nid, PARAM_AMP_IN_CAP)? } else { 0 };
        w.amp_out = if w.has_out_amp() { param(link, cad, nid, PARAM_AMP_OUT_CAP)? } else { 0 };
    } else {
        w.amp_in = afg.0;
        w.amp_out = afg.1;
    }
    if w.wcaps & WCAP_CONN_LIST != 0 {
        let len = param(link, cad, nid, PARAM_CONNLIST_LEN)?;
        let mut failed = None;
        let mut conn = [0u8; super::widget::MAX_CONN];
        w.n_conn = decode(
            len,
            |i| match link.send(compose_verb(cad, nid, VERB_GET_CONNECT_LIST, i as u16)) {
                Ok(v) => Some(v),
                Err(e) => {
                    failed = Some(e);
                    None
                }
            },
            &mut conn,
        );
        if let Some(e) = failed {
            return Err(e);
        }
        w.conn = conn;
    }
    match w.ty() {
        WIDGET_TYPE_PIN => {
            w.pin_caps = param(link, cad, nid, PARAM_PIN_CAP)?;
            w.pin_cfg = PinConfig(link.send(compose_verb(cad, nid, VERB_GET_CONFIG_DEFAULT, 0))?);
        }
        WIDGET_TYPE_DAC => {
            // Without the format override bit the converter takes the
            // function group's formats (Linux `query_stream_param`).
            w.pcm = if w.wcaps & WCAP_FORMAT_OVRD != 0 {
                param(link, cad, nid, PARAM_PCM)?
            } else {
                afg.2
            };
        }
        _ => {}
    }
    Ok(w)
}
