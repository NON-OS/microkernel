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

//! The tick's half of a Wi-Fi scan or join (`wifi_pending.rs`).

use nonos_libc::{mk_time_millis, Look};

use super::refresh_wifi::scan_now;
use super::state::State;
use super::wifi_answer::apply;
use super::wifi_join::{clear_passphrase, join_now};
use super::wifi_pending::{Next, Pending, Work};
use super::wifi_worker::poll;

/// Advance a pending scan or join by one tick: show it, hand it to the
/// worker, or, once it is out, look for the answer. True when the panel
/// changed and wants painting.
pub fn step_wifi(state: &mut State) -> bool {
    if state.wifi.pending.is_out() {
        return match poll() {
            Look::Waiting => false,
            Look::Ready(answer) => {
                state.wifi.pending = Pending::Idle;
                apply(state, answer);
                true
            }
            Look::Idle => {
                state.wifi.pending = Pending::Idle;
                state.wifi.notice = Some("The driver's answer was lost; ask again");
                true
            }
        };
    }
    match state.wifi.pending.tick(mk_time_millis()) {
        Next::Nothing => false,
        Next::Show => true,
        Next::Run(Work::Scan) => {
            scan_now(state);
            true
        }
        Next::Run(Work::Join) => {
            join_now(state);
            true
        }
        Next::Expired(work) => {
            if work == Work::Join {
                clear_passphrase(state);
            }
            state.wifi.notice = Some("Timed out before it ran; ask again");
            true
        }
    }
}
