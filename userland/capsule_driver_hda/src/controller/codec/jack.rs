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
//! The headphone jack: whether something is plugged in, and which outputs
//! play because of it.
//!
//! The pin's presence detect is read with GET_PIN_SENSE (bit 31), after an
//! Execute (SET_PIN_SENSE) on a pin whose capabilities say a trigger is
//! required. The jack is polled from the serving loop rather than reported
//! by unsolicited response; Linux falls back to the same polling
//! (`jackpoll_ms`) where unsolicited events are unreliable. With headphones
//! in, the speakers' pin output is switched off, as Linux's auto-mute does
//! (`snd_hda_gen_update_outputs`); the headphone and line outs stay on.

use super::pincfg::OutKind;
use super::plan::Plan;
use super::widget::Codec;
use crate::constants::{
    PINCAP_HP_DRV, PINCAP_TRIG_REQ, PIN_HP_ENABLE, PIN_OUT_ENABLE, PIN_SENSE_PRESENT,
    VERB_GET_PIN_SENSE, VERB_SET_PIN_SENSE, VERB_SET_PIN_WIDGET_CONTROL,
};
use crate::controller::compose_verb;
use crate::controller::verb::Link;
use crate::error::HdaResult;

/// The pin widget control an output gets with headphones `plugged` or not.
pub const fn pin_ctl(kind: OutKind, hp_drive: bool, plugged: bool) -> u8 {
    match kind {
        OutKind::Speaker if plugged => 0,
        OutKind::Headphone if hp_drive => PIN_OUT_ENABLE | PIN_HP_ENABLE,
        _ => PIN_OUT_ENABLE,
    }
}

pub fn sense(link: &mut Link, codec: &Codec, pin: u8) -> HdaResult<bool> {
    let trig = codec.get(pin).is_some_and(|w| w.pin_caps & PINCAP_TRIG_REQ != 0);
    if trig {
        link.send(compose_verb(codec.cad, pin, VERB_SET_PIN_SENSE, 0))?;
    }
    let r = link.send(compose_verb(codec.cad, pin, VERB_GET_PIN_SENSE, 0))?;
    Ok(r & PIN_SENSE_PRESENT != 0)
}

/// Program every output's pin control for `plugged`.
pub fn apply(link: &mut Link, codec: &Codec, plan: &Plan, plugged: bool) -> HdaResult<()> {
    for o in plan.iter() {
        let drive = codec.get(o.pin()).is_some_and(|w| w.pin_caps & PINCAP_HP_DRV != 0);
        let ctl = pin_ctl(o.kind, drive, plugged);
        link.send(compose_verb(codec.cad, o.pin(), VERB_SET_PIN_WIDGET_CONTROL, ctl as u16))?;
    }
    Ok(())
}

/// Whether headphones are in, read from the plan's sensing jack. A plan
/// without one has nothing to switch and reads as unplugged.
pub fn plugged(link: &mut Link, codec: &Codec, plan: &Plan) -> HdaResult<bool> {
    match plan.sensing_headphone() {
        Some(pin) if plan.has(OutKind::Speaker) => sense(link, codec, pin),
        _ => Ok(false),
    }
}
