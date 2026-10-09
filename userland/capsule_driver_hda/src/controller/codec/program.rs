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
//! The verbs that turn a planned set of outputs into sound.
//!
//! The driver used to send seven: D0 to the group, the DAC and the pin, the
//! pin's output enable, an unmute with gain 0x7f on the DAC, the format and
//! the stream. On a real codec that leaves the speaker silent in several
//! ways at once: the mixer between DAC and pin keeps its inputs muted, the
//! pin's own output amp stays muted, a selector or a pin with more than one
//! input keeps whatever input it reset to, the external amplifier behind
//! EAPD stays off, and a gain above the amp's step count is undefined. Here
//! each widget on each path is set the way Linux's generic parser sets it
//! (`activate_path` and `init_amp`, hda_generic.c): the connection selected,
//! the input amp of that connection unmuted on a mixer with the others
//! muted, every output amp unmuted at its 0 dB step (the offset in its amp
//! capabilities), then EAPD on every pin that has it (`alc_auto_setup_eapd`)
//! and the converters joined to the stream.
//!
//! The output amps are left muted at their 0 dB step, and any beep
//! generator widget is switched off and muted. Nothing reaches the speakers
//! until a player starts a stream and `set_outputs` opens them; stopping the
//! stream closes them again. A codec that powers up with its amps open would
//! otherwise play whatever the DMA engine or the PC beep line carries from
//! the moment the driver starts.

use super::format::PLAYBACK;
use super::jack;
use super::plan::Plan;
use super::realtek;
use super::widget::{Codec, Widget};
use crate::constants::{
    AMPCAP_OFFSET, AMP_IN_UNMUTE, AMP_MUTE, AMP_OUT_UNMUTE, EAPD_ENABLE, PINCAP_EAPD,
    VERB_SET_AMP_GAIN_MUTE, VERB_SET_CHANNEL_STREAMID, VERB_SET_CONNECT_SEL,
    VERB_SET_EAPD_BTLENABLE, VERB_SET_GPIO_DATA, VERB_SET_GPIO_DIRECTION, VERB_SET_GPIO_MASK,
    VERB_SET_BEEP_CONTROL, VERB_SET_STREAM_FORMAT, WIDGET_TYPE_BEEP, WIDGET_TYPE_MIXER,
    WIDGET_TYPE_PIN,
};
use crate::controller::verb::Link;
use crate::controller::{compose_verb, compose_verb_long};
use crate::error::HdaResult;

/// HP's PCI subsystem vendor, as the codec's GET_SUBSYSTEM_ID reports it.
pub const SUBSYSTEM_HP: u16 = 0x103c;

/// The gain step that is 0 dB on an amp with capabilities `caps`, held to
/// the steps the amp has.
pub const fn zero_db(caps: u32) -> u16 {
    let offset = caps & AMPCAP_OFFSET;
    let steps = (caps >> 8) & 0x7f;
    (if offset > steps { steps } else { offset }) as u16
}

/// The GPIO mask, direction and data for codec `subsystem_id` with
/// `count` GPIOs, or none. On HP machines Linux's Realtek fixups drive
/// their GPIOs as outputs, high (`alc_setup_gpio`): an amplifier enable is
/// on when high (`alc_fixup_gpio1` to `4`) and the mute and mic-mute LEDs
/// are off when high (`alc_update_gpio_led`). Driving every GPIO the codec
/// has that way is the one setting right on all of them.
pub const fn hp_gpio(subsystem_id: u32, count: u8) -> Option<u8> {
    if (subsystem_id >> 16) as u16 != SUBSYSTEM_HP || count == 0 {
        return None;
    }
    Some(if count >= 8 { 0xff } else { (1u8 << count) - 1 })
}

/// Program the codec for `plan` on stream `tag`. Returns whether headphones
/// were in, read once the codec is powered (a pin in D3 senses nothing).
pub fn program(link: &mut Link, codec: &Codec, plan: &Plan, tag: u8) -> HdaResult<bool> {
    let cad = codec.cad;
    super::power::up(link, codec)?;
    let plugged = jack::plugged(link, codec, plan)?;
    let hp_pin = plan
        .iter()
        .find(|o| o.kind == super::pincfg::OutKind::Headphone)
        .map(|o| o.pin())
        .unwrap_or(0x21);
    realtek::init(link, codec, hp_pin, plugged)?;
    silence_beep(link, codec)?;
    for o in plan.iter() {
        for (i, (nid, sel)) in o.path.hops().enumerate() {
            let Some(w) = codec.get(nid) else { continue };
            let last = i + 1 == o.path.len as usize;
            if !last {
                route(link, cad, w, sel)?;
            }
        }
    }
    set_outputs(link, codec, plan, false)?;
    jack::apply(link, codec, plan, plugged)?;
    for w in codec.all() {
        if w.ty() == WIDGET_TYPE_PIN && w.pin_caps & PINCAP_EAPD != 0 {
            link.send(compose_verb(cad, w.nid, VERB_SET_EAPD_BTLENABLE, EAPD_ENABLE as u16))?;
        }
    }
    if let Some(bits) = hp_gpio(codec.subsystem_id, codec.gpio_count) {
        let afg = codec.afg;
        link.send(compose_verb(cad, afg, VERB_SET_GPIO_MASK, bits as u16))?;
        link.send(compose_verb(cad, afg, VERB_SET_GPIO_DIRECTION, bits as u16))?;
        crate::clock::pause_ms(1);
        link.send(compose_verb(cad, afg, VERB_SET_GPIO_DATA, bits as u16))?;
    }
    join_stream(link, codec, plan, tag)?;
    Ok(plugged)
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

/// Switch off every beep generator widget (type 7) and mute its output amp,
/// so a codec's own beep cannot sound (HDA 1.0a section 7.3.3.15: a divider
/// of 0 disables the generator).
fn silence_beep(link: &mut Link, codec: &Codec) -> HdaResult<()> {
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

/// Select input `sel` on a widget that passes one input, or open input
/// `sel`'s amp and close the others on a mixer.
fn route(link: &mut Link, cad: u8, w: &Widget, sel: u8) -> HdaResult<()> {
    let nid = w.nid;
    if w.ty() == WIDGET_TYPE_MIXER {
        if w.has_in_amp() {
            for i in 0..w.n_conn {
                let v = if i == sel {
                    AMP_IN_UNMUTE | ((i as u16) << 8) | zero_db(w.amp_in)
                } else {
                    AMP_IN_UNMUTE | ((i as u16) << 8) | AMP_MUTE
                };
                link.send(compose_verb_long(cad, nid, VERB_SET_AMP_GAIN_MUTE as u16, v))?;
            }
        }
        return Ok(());
    }
    if w.n_conn > 1 {
        link.send(compose_verb(cad, nid, VERB_SET_CONNECT_SEL, sel as u16))?;
    }
    if w.has_in_amp() && w.ty() != WIDGET_TYPE_PIN {
        let v = AMP_IN_UNMUTE | ((sel as u16) << 8) | zero_db(w.amp_in);
        link.send(compose_verb_long(cad, nid, VERB_SET_AMP_GAIN_MUTE as u16, v))?;
    }
    Ok(())
}

/// Format and stream on every DAC the plan uses, once each. One stream may
/// feed several converters (HDA 1.0a section 4.5.3); each takes channels 0
/// and 1 of it.
fn join_stream(link: &mut Link, codec: &Codec, plan: &Plan, tag: u8) -> HdaResult<()> {
    let mut done = [0u8; super::plan::MAX_OUTPUTS];
    let mut n = 0usize;
    for o in plan.iter() {
        let dac = o.path.dac();
        if done[..n].contains(&dac) {
            continue;
        }
        done[n] = dac;
        n += 1;
        let cad = codec.cad;
        link.send(compose_verb_long(cad, dac, VERB_SET_STREAM_FORMAT as u16, PLAYBACK))?;
        link.send(compose_verb(cad, dac, VERB_SET_CHANNEL_STREAMID, (tag as u16) << 4))?;
    }
    Ok(())
}
