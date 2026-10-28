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

//! The streams waiting for a circuit whose exit takes their port, each turn.

use crate::stream::StreamStage;
use crate::trace;

use super::super::state::Manager;
use super::attach::attach;
use super::pick_circuit::pick;
use super::room::make_room;
use super::want::{Want, WEB};

/// Send each waiting stream's open down a circuit whose exit takes its port,
/// end one whose wait is over, and say what the next general circuit's exit
/// must take: the first port still waited on, or the web ports.
pub fn waiting(state: &mut Manager, now: u64) -> Want {
    let mut want = None;
    for at in 0..state.streams.len() {
        let stream = &mut state.streams[at];
        let Some(held) = stream.held.as_ref() else { continue };
        if stream.circuit != 0 || stream.stage != StreamStage::Opening {
            continue;
        }
        if now >= held.until {
            stream.stage = StreamStage::Ended(held.why);
            stream.held = None;
            trace::say_num(b"stream found no exit for its port in time", stream.id as u64);
            continue;
        }
        let (port, install) = (held.port, held.install);
        if !pick(state, now, port, install).is_some_and(|index| attach(state, at, index)) {
            want = want.or(Some((port, install)));
        }
    }
    let Some((port, install)) = want else { return Want { port: WEB, waited: false } };
    make_room(state, port, install);
    Want { port, waited: true }
}
