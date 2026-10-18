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

//! Each widget on a path set to pass the path (Linux `activate_path`,
//! hda_generic.c).

use super::amp::zero_db;
use super::widget::Widget;
use crate::constants::{
    AMP_INDEX_COUNT, AMP_IN_UNMUTE, AMP_MUTE, VERB_SET_AMP_GAIN_MUTE, VERB_SET_CONNECT_SEL,
    WIDGET_TYPE_MIXER, WIDGET_TYPE_PIN,
};
use crate::controller::verb::Link;
use crate::controller::{compose_verb, compose_verb_long};
use crate::error::HdaResult;

/// Select input `sel` on a widget that passes one input, or open input
/// `sel`'s amp and close the others on a mixer. Only the first sixteen input
/// amps can be named (`AMP_INDEX_COUNT`): an index past them would wrap onto
/// another input and open or close the wrong one, so none is sent for them,
/// and the search never routes through one.
pub fn route(link: &mut Link, cad: u8, w: &Widget, sel: u8) -> HdaResult<()> {
    let nid = w.nid;
    let amp = |i: u8, v: u16| {
        compose_verb_long(cad, nid, VERB_SET_AMP_GAIN_MUTE as u16, v | (i as u16) << 8)
    };
    if w.ty() == WIDGET_TYPE_MIXER {
        if w.has_in_amp() {
            for i in 0..w.n_conn.min(AMP_INDEX_COUNT) {
                let gain = if i == sel { zero_db(w.amp_in) } else { AMP_MUTE };
                link.send(amp(i, AMP_IN_UNMUTE | gain))?;
            }
        }
        return Ok(());
    }
    if w.n_conn > 1 {
        link.send(compose_verb(cad, nid, VERB_SET_CONNECT_SEL, sel as u16))?;
    }
    if w.has_in_amp() && w.ty() != WIDGET_TYPE_PIN && sel < AMP_INDEX_COUNT {
        link.send(amp(sel, AMP_IN_UNMUTE | zero_db(w.amp_in)))?;
    }
    Ok(())
}
