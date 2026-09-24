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

//! A control frame from the gateway, which arrives in the clear.

use super::control::note_control;
use crate::gateway_client;
use crate::trace;

/// Handle one clear-text frame.
pub(super) fn control(tcp_port: u32, stream: u32, frame: &[u8]) {
    if !note_control(frame) {
        return;
    }
    /*
     * Allowance is granted per session and spent per packet, so running out is
     * a state to leave rather than a failure to report.
     */
    let _ = gateway_client::claim_free_bandwidth(tcp_port, stream);
    trace::say(b"asked the gateway for allowance again");
}
