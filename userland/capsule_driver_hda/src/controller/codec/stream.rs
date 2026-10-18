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

//! The converters joined to the stream.

use super::format::PLAYBACK;
use super::plan::{Plan, MAX_OUTPUTS};
use super::widget::Codec;
use crate::constants::{VERB_SET_CHANNEL_STREAMID, VERB_SET_STREAM_FORMAT};
use crate::controller::verb::Link;
use crate::controller::{compose_verb, compose_verb_long};
use crate::error::HdaResult;

/// Format and stream on every DAC the plan uses, once each. One stream may
/// feed several converters (HDA 1.0a section 4.5.3); each takes channels 0
/// and 1 of it.
pub(super) fn join(link: &mut Link, codec: &Codec, plan: &Plan, tag: u8) -> HdaResult<()> {
    let mut done = [0u8; MAX_OUTPUTS];
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
