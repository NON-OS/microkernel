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

//! The two lines naming what the host says it can do, read once at init.

use super::super::super::env::{Clock, Log, Mmio};
use super::super::super::text::Line;
use super::engine::Host;

impl<M: Mmio, C: Clock, L: Log> Host<M, C, L> {
    pub(super) fn say_caps(&self) {
        let c = self.caps;
        self.say(
            Line::new()
                .s(b"host spec ")
                .dec(c.spec() as u64)
                .s(b" vendor ")
                .hex((c.version >> 8) as u64)
                .s(b" caps ")
                .hex(c.caps as u64)
                .s(b" caps1 ")
                .hex(c.caps1 as u64),
        );
        self.say(
            Line::new()
                .s(b"base clock ")
                .dec((c.base_hz() / 1_000_000) as u64)
                .s(if c.base_given() { b" MHz" as &[u8] } else { b" MHz (not given, assumed)" })
                .s(b" slot type ")
                .dec(c.slot_type() as u64)
                .s(b" adma2 ")
                .dec(c.adma2() as u64)
                .s(b" dma64 ")
                .dec(c.dma64() as u64)
                .s(b" 8bit ")
                .dec(c.bus8() as u64)
                .s(b" hs ")
                .dec(c.high_speed() as u64),
        );
    }
}
