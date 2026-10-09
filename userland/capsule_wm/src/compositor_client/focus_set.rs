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

use super::wire::{call, TIMED_OUT};
use crate::state::restack::Outcome;

const OP: u16 = 0x0004;
const BODY_LEN: usize = 8;

/// Tell the compositor `target_pid` has focus, which lifts its layers. A
/// late answer is a delivered request (wire.rs `TIMED_OUT`); a refusal or a
/// call that never reached it is lost.
pub fn push_focus_set(compositor_port: u32, request_id: u32, target_pid: u32) -> Outcome {
    let mut body = [0u8; BODY_LEN];
    body[0..4].copy_from_slice(&target_pid.to_le_bytes());
    match call(compositor_port, OP, request_id, &body) {
        Ok(0) => Outcome::Answered,
        Err(e) if e == TIMED_OUT => Outcome::Late,
        _ => Outcome::Lost,
    }
}
