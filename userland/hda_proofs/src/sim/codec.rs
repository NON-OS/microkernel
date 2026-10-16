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

//! A codec as the model holds it: each node's parameters, connection list
//! and pin configuration.

use std::collections::HashMap;

pub const WCAP_VENDOR: u32 = 0xf << 20;

#[derive(Clone, Default)]
pub struct Node {
    pub params: HashMap<u16, u32>,
    pub conn: Vec<u8>,
    pub pin_cfg: u32,
}

#[derive(Clone, Default)]
pub struct SimCodec {
    pub nodes: HashMap<u8, Node>,
    pub subsystem: u32,
    /// Unsolicited responses sent ahead of the answer to every command.
    pub unsol_before_each: u32,
}

impl SimCodec {
    pub fn node(&mut self, nid: u8) -> &mut Node {
        self.nodes.entry(nid).or_default()
    }

    pub fn param(&mut self, nid: u8, p: u16, v: u32) -> &mut Self {
        self.node(nid).params.insert(p, v);
        self
    }
}
