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

//! The codec's own beep generator, kept silent.

use super::amp::out_amp;
use super::widget::Codec;
use crate::constants::{VERB_SET_AMP_GAIN_MUTE, VERB_SET_BEEP_CONTROL, WIDGET_TYPE_BEEP};
use crate::controller::verb::Link;
use crate::controller::{compose_verb, compose_verb_long};
use crate::error::HdaResult;

/// Switch off every beep generator widget (type 7) and mute its output amp,
/// so a codec's own beep cannot sound (HDA 1.0a section 7.3.3.15: a divider
/// of 0 disables the generator).
pub(super) fn silence(link: &mut Link, codec: &Codec) -> HdaResult<()> {
    let cad = codec.cad;
    for w in codec.all() {
        if w.ty() != WIDGET_TYPE_BEEP {
            continue;
        }
        link.send(compose_verb(cad, w.nid, VERB_SET_BEEP_CONTROL, 0))?;
        if w.has_out_amp() {
            let v = out_amp(w.amp_out, false);
            link.send(compose_verb_long(cad, w.nid, VERB_SET_AMP_GAIN_MUTE as u16, v))?;
        }
    }
    Ok(())
}
