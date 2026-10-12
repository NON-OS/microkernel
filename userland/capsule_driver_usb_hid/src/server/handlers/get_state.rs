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

use crate::hid::{key_post_failures, pointer_post_failures};
use crate::protocol::{Request, HDR_LEN, STATUS_LEN};
use crate::server::respond;
use crate::state::State;

pub fn handle(state: &State, sender_pid: u32, req: &Request, tx: &mut [u8]) {
    let base = HDR_LEN + STATUS_LEN;
    tx[base..base + 8].copy_from_slice(&state.configs_probed.to_le_bytes());
    tx[base + 8..base + 16].copy_from_slice(&state.key_reports.to_le_bytes());
    tx[base + 16..base + 24].copy_from_slice(&state.mouse_reports.to_le_bytes());
    // Bytes 24 to 31 were the depths of the key and mouse event queues,
    // which kept what was typed and are gone; they stay, zero, so the
    // reply keeps its layout.
    tx[base + 24..base + 32].fill(0);
    tx[base + 32..base + 40].copy_from_slice(&key_post_failures().to_le_bytes());
    tx[base + 40..base + 48].copy_from_slice(&pointer_post_failures().to_le_bytes());
    let _ = respond::payload(sender_pid, req, 48, tx);
}
