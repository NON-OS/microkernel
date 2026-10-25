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

use crate::settings::schema::rows::Tone;
use crate::settings::state::State;
use crate::wifi::{Lease, NetStatus};

use super::valbuf::ValBuf;
use crate::wifi::station::said;

pub fn link_state(state: &State) -> (ValBuf, Tone) {
    let mut b = ValBuf::new();
    let tone = match state.wifi_diag.net {
        NetStatus::Bound { .. } => {
            b.push_str("Connected");
            Tone::Ok
        }
        NetStatus::Unbound { .. } => {
            b.push_str("No address");
            Tone::Warn
        }
        NetStatus::NoReply => {
            b.push_str("Not responding");
            Tone::Warn
        }
        NetStatus::NoService => {
            b.push_str("Offline");
            Tone::Idle
        }
    };
    (b, tone)
}

pub fn addr(b: &mut ValBuf, state: &State, pick: fn(&Lease) -> [u8; 4]) -> Tone {
    match &state.wifi_diag.net {
        NetStatus::Bound { lease, .. } => b.push_ipv4(pick(lease)),
        _ => b.push_str("--"),
    }
    Tone::Idle
}

pub fn adapter(b: &mut ValBuf, state: &State) -> Tone {
    if state.wifi_adapter_count == 0 {
        b.push_str("None detected");
        return Tone::Idle;
    }
    b.push_bytes(state.wifi_adapters[0].name());
    Tone::Idle
}

/// Where the wired card's address came from, in the page's words; nothing
/// when its driver does not say (wifi/station.rs).
pub fn station(b: &mut ValBuf, state: &State) -> Tone {
    b.push_str(state.wifi_diag.station.and_then(said).map_or("", |s| s.0));
    Tone::Idle
}

/// The line under the address row, when its value needs one.
pub fn station_note(state: &State) -> Option<&'static str> {
    state.wifi_diag.station.and_then(said).and_then(|s| s.1)
}
