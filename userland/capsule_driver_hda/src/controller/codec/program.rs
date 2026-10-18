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
//! The driver used to send seven, and a real codec stayed silent: mixer
//! inputs and the pin's output amp muted, a selector on its reset input,
//! the amplifier behind EAPD off. Here each widget on each path is set the
//! way Linux's generic parser sets it (`activate_path` and `init_amp`,
//! hda_generic.c; `route`), every output amp at its 0 dB step (`amp`), then
//! EAPD and the HP GPIOs (`external`) and the stream (`stream`). The output
//! amps stay muted and any beep generator off (`beep`) until a player starts
//! a stream and `set_outputs` opens them; stopping the stream closes them.

pub use super::amp::set_outputs;
use super::pincfg::OutKind;
use super::plan::Plan;
use super::route::route;
use super::widget::Codec;
use super::{beep, external, jack, power, realtek, stream};
use crate::controller::verb::Link;
use crate::error::HdaResult;

/// HP's PCI subsystem vendor, as the codec's GET_SUBSYSTEM_ID reports it.
pub const SUBSYSTEM_HP: u16 = 0x103c;

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
    power::up(link, codec)?;
    let plugged = jack::plugged(link, codec, plan)?;
    let hp = plan.iter().find(|o| o.kind == OutKind::Headphone);
    realtek::init(link, codec, hp.map(|o| o.pin()).unwrap_or(0x21), plugged)?;
    beep::silence(link, codec)?;
    for o in plan.iter() {
        // Every hop but the DAC passes the path on.
        for (nid, sel) in o.path.hops().take((o.path.len as usize).saturating_sub(1)) {
            let Some(w) = codec.get(nid) else { continue };
            route(link, codec.cad, w, sel)?;
        }
    }
    set_outputs(link, codec, plan, false)?;
    jack::apply(link, codec, plan, plugged)?;
    external::enable(link, codec)?;
    stream::join(link, codec, plan, tag)?;
    Ok(plugged)
}
