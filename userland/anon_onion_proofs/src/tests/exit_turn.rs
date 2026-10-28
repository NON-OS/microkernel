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

//! Turns of Net's manager, and what a stream has come to.

use std::string::String;

use crate::manager::circuit_tick;
use crate::stream::StreamStage;

use super::exit_net::{Net, T0};

impl Net {
    /// The circuit stream `id` is on, 0 while it waits, and its stage named.
    pub fn stream(&self, id: u16) -> (u32, String) {
        let s = self.state.streams.iter().find(|s| s.id == id).expect("in the table");
        let stage = match s.stage {
            StreamStage::Opening => "opening".into(),
            StreamStage::Open => "open".into(),
            StreamStage::Ended(reason) => std::format!("ended {reason}"),
        };
        (s.circuit, stage)
    }

    /// An idle turn of circuit_tick, `seconds` on.
    pub fn tick(&mut self, seconds: u64) {
        self.now += seconds;
        nonos_libc::set_ms(1_000_000 + (self.now - T0) as i64 * 1000);
        circuit_tick(&mut self.state, self.now);
    }
}
