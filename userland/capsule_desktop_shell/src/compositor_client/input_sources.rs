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

//! NCMP 0x0017 INPUT_SOURCES and SET_VCP's input source (0x0015): the
//! inputs each display lists, and one display switched to one of them on a
//! choice the input menu permits.

use super::wire::{call, call_payload_within, CONTROL_REPLY_TIMEOUT_MS};
use crate::state::input_menu::Switch;
use crate::state::input_offers::ANSWER_LEN;

const OP_INPUT_SOURCES: u16 = 0x0017;
const OP_SET_VCP: u16 = 0x0015;
/// MCCS's input source control.
const INPUT_SOURCE: u32 = 0x60;

/// The compositor's answer, when it gave one with status 0.
pub fn offered(compositor_port: u32, request_id: u32) -> Option<[u8; ANSWER_LEN]> {
    let mut out = [0u8; ANSWER_LEN];
    let wait = CONTROL_REPLY_TIMEOUT_MS;
    let status =
        call_payload_within(compositor_port, OP_INPUT_SOURCES, request_id, &[], &mut out, wait);
    (status.ok()? == 0).then_some(out)
}

/// Switch as `go` permits, consuming it; the compositor's status.
pub fn switch(compositor_port: u32, request_id: u32, go: Switch) -> Result<i32, &'static str> {
    let mut body = [0u8; 12];
    body[..4].copy_from_slice(&go.identity().to_le_bytes());
    body[4..8].copy_from_slice(&INPUT_SOURCE.to_le_bytes());
    body[8..].copy_from_slice(&u32::from(go.input()).to_le_bytes());
    call(compositor_port, OP_SET_VCP, request_id, &body)
}
