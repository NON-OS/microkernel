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

//! Telling the input router the display size (`state::display_rule`), NIRS
//! op 0x0006, body width and height as u32s: the canvas DISPLAY_INFO answers
//! with, so the pointer is placed on what is drawn.

use crate::state::display_rule::{due, RETRY_MS};
use crate::state::Context;

const OP_DISPLAY_SIZE: u16 = 0x0006;

pub struct DisplayNotice {
    told: Option<(u32, u32)>,
    last_ms: i64,
}

impl DisplayNotice {
    pub fn new(now_ms: i64) -> Self {
        // As if a notice went one retry ago: the first size is told at once.
        DisplayNotice { told: None, last_ms: now_ms.saturating_sub(RETRY_MS) }
    }

    pub fn run(&mut self, ctx: &Context, now_ms: i64) {
        let size = (ctx.width, ctx.height);
        if !due(size, self.told, self.last_ms, now_ms) {
            return;
        }
        self.last_ms = now_ms;
        let mut body = [0u8; 8];
        body[..4].copy_from_slice(&size.0.to_le_bytes());
        body[4..].copy_from_slice(&size.1.to_le_bytes());
        if super::router_send::send(OP_DISPLAY_SIZE, &body) {
            self.told = Some(size);
        }
    }
}
