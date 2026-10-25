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

//! Telling the input router who holds the trusted band (`trusted_rule`).
//! One-way, NIRS v1 op 0x0005, body the holder's pid u32 (0: none): the
//! router also calls the compositor, so a call each way could stall both.
//! The router takes it from the compositor's pid only, and a lost notice is
//! made good by the next one, at most a second later.

use crate::state::trusted_rule::{due, holder};
use crate::state::Context;

const OP_TRUSTED_HOLD: u16 = 0x0005;

pub struct Notice {
    told: Option<u32>,
    last_ms: i64,
}

impl Notice {
    pub fn new(now_ms: i64) -> Self {
        Notice { told: None, last_ms: now_ms }
    }

    pub fn run(&mut self, ctx: &Context, now_ms: i64) {
        let held = holder(ctx.scene.layers());
        if !due(held, self.told, self.last_ms, now_ms) {
            return;
        }
        self.last_ms = now_ms;
        if send(held.unwrap_or(0)) {
            self.told = held;
        }
    }
}

fn send(pid: u32) -> bool {
    super::router_send::send(OP_TRUSTED_HOLD, &pid.to_le_bytes())
}
