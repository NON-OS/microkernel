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

//! One widget's parameters, connection list and pin or converter details.

use super::{conn::decode, pincfg::PinConfig, walk_afg::param, widget::Widget};
use crate::constants::{
    PARAM_AMP_IN_CAP, PARAM_AMP_OUT_CAP, PARAM_AUDIO_WIDGET_CAP, PARAM_CONNLIST_LEN, PARAM_PCM,
    PARAM_PIN_CAP, VERB_GET_CONFIG_DEFAULT, VERB_GET_CONNECT_LIST, WCAP_AMP_OVRD, WCAP_CONN_LIST,
    WCAP_FORMAT_OVRD, WIDGET_TYPE_DAC, WIDGET_TYPE_PIN,
};
use crate::controller::{compose_verb, verb::Link};
use crate::error::HdaResult;

pub(super) fn widget(link: &mut Link, cad: u8, nid: u8, afg: (u32, u32, u32)) -> HdaResult<Widget> {
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
