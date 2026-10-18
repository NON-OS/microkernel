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

//! Amp settings: the 0 dB step each amp's capabilities name, and the output
//! amps on every planned path, closed until a stream plays.

use super::plan::Plan;
use super::widget::Codec;
use crate::constants::{AMPCAP_OFFSET, AMP_MUTE, AMP_OUT_UNMUTE, VERB_SET_AMP_GAIN_MUTE};
use crate::controller::compose_verb_long;
use crate::controller::verb::Link;
use crate::error::HdaResult;

/// The gain step that is 0 dB on an amp with capabilities `caps`, held to
/// the steps the amp has.
pub const fn zero_db(caps: u32) -> u16 {
    let offset = caps & AMPCAP_OFFSET;
    let steps = (caps >> 8) & 0x7f;
    (if offset > steps { steps } else { offset }) as u16
}

/// The SET_AMP_GAIN_MUTE payload for an output amp with capabilities
/// `caps`: both channels at the 0 dB step, muted unless `open`.
pub const fn out_amp(caps: u32, open: bool) -> u16 {
    let v = AMP_OUT_UNMUTE | zero_db(caps);
    if open {
        v
    } else {
        v | AMP_MUTE
    }
}

/// Open or close every output amp on every planned path. Closed at init and
/// when a stream stops, open while a stream plays.
pub fn set_outputs(link: &mut Link, codec: &Codec, plan: &Plan, open: bool) -> HdaResult<()> {
    let cad = codec.cad;
    for o in plan.iter() {
        for (nid, _) in o.path.hops() {
            let Some(w) = codec.get(nid) else { continue };
            if w.has_out_amp() {
                let v = out_amp(w.amp_out, open);
                link.send(compose_verb_long(cad, nid, VERB_SET_AMP_GAIN_MUTE as u16, v))?;
            }
        }
    }
    Ok(())
}
