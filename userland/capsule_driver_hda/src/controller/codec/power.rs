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

//! Powering a codec's function group and its widgets to D0.
//!
//! After a link reset a codec may sit in D3. A widget in D3 answers every
//! verb and keeps none of them, so the whole configuration lands nowhere if
//! it arrives first. Linux (`hda_set_power_state`, hda_codec.c) sets the
//! function group to D0, sets every widget that has its own power control,
//! then reads GET_POWER_STATE until the state reached (bits 7:4) is D0,
//! for up to 500 ms, trying the whole step again up to ten times while the
//! codec reports the error bit.

use super::widget::Codec;
use crate::clock::pause_ms;
use crate::constants::{
    POWER_ACTUAL_SHIFT, POWER_D0, POWER_ERROR, VERB_GET_POWER_STATE, VERB_SET_POWER_STATE,
};
use crate::controller::compose_verb;
use crate::controller::verb::Link;
use crate::controller::wait::Wait;
use crate::error::{HdaError, HdaResult};

const SYNC_MS: u64 = 500;
const TRIES: u32 = 10;

/// What a GET_POWER_STATE answer says, for the target `D0`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Reached {
    Yes,
    NotYet,
    Error,
}

pub const fn reached(resp: u32) -> Reached {
    if resp & POWER_ERROR != 0 {
        Reached::Error
    } else if (resp >> POWER_ACTUAL_SHIFT) & 0xf == POWER_D0 as u32 {
        Reached::Yes
    } else {
        Reached::NotYet
    }
}

pub fn up(link: &mut Link, codec: &Codec) -> HdaResult<()> {
    let cad = codec.cad;
    let mut tries = 0u32;
    while tries < TRIES {
        link.send(compose_verb(cad, codec.afg, VERB_SET_POWER_STATE, POWER_D0 as u16))?;
        for w in codec.all().iter().filter(|w| w.has_power()) {
            link.send(compose_verb(cad, w.nid, VERB_SET_POWER_STATE, POWER_D0 as u16))?;
        }
        let mut wait = Wait::ms(SYNC_MS);
        loop {
            let r = link.send(compose_verb(cad, codec.afg, VERB_GET_POWER_STATE, 0))?;
            match reached(r) {
                Reached::Yes => return Ok(()),
                Reached::Error => break,
                Reached::NotYet if wait.expired() => return Err(HdaError::CodecPowerTimeout),
                Reached::NotYet => pause_ms(1),
            }
        }
        tries += 1;
    }
    Err(HdaError::CodecPowerTimeout)
}
