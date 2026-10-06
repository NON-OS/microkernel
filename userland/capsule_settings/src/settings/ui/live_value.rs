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

use crate::settings::schema::rows::{Live, Tone};
use crate::settings::state::audio_output;
use crate::settings::state::machine_key::said;
use crate::settings::state::State;

use super::build_info::{ARCHITECTURE, GIT_SHA, TOOLCHAIN, VERSION};
use super::live_net::{adapter, addr, link_state};
use super::live_wifi::{join, link, remember};
use super::valbuf::ValBuf;

/// Format one live row's value, and the tone it should read in.
pub fn resolve(state: &State, live: Live) -> (ValBuf, Tone) {
    let mut b = ValBuf::new();
    let tone = match live {
        Live::LinkState => return link_state(state),
        Live::IpAddress => addr(&mut b, state, |l| l.ip),
        Live::Gateway => addr(&mut b, state, |l| l.gw),
        Live::Dns => addr(&mut b, state, |l| l.dns),
        Live::Adapter => adapter(&mut b, state),
        Live::Version => text(&mut b, VERSION.trim()),
        Live::Commit => text(&mut b, GIT_SHA),
        Live::Toolchain => text(&mut b, TOOLCHAIN),
        Live::Architecture => text(&mut b, ARCHITECTURE),
        Live::WifiLink => link(&mut b, state),
        Live::WifiJoin => join(&mut b, state),
        Live::WifiRemember => remember(&mut b, state),
        Live::MachineKey => {
            let (words, tone) = said(state.machine_key);
            b.push_str(words);
            tone
        }
        Live::AudioOutput => {
            let (words, tone, _) = audio_output::said(state.audio_output);
            b.push_str(words);
            tone
        }
    };
    (b, tone)
}

/// The line under a live row's label, for the rows that carry one. The
/// output device row always does, so its height does not change with what
/// the hardware said: the reason sound cannot play is too long for the
/// value column.
pub fn note(state: &State, live: Live) -> Option<&'static str> {
    match live {
        Live::AudioOutput => Some(audio_output::said(state.audio_output).2),
        _ => None,
    }
}

fn text(b: &mut ValBuf, s: &str) -> Tone {
    b.push_str(s);
    Tone::Idle
}
