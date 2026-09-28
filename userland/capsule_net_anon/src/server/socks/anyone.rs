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

//! The Anyone network as what carries a SOCKS conversation.

extern crate alloc;

use alloc::vec::Vec;

use crate::manager::{send_data, Manager, SendError};
use crate::stream::StreamStage;

use super::tunnel::{Far, Tunnel, Unsent};

pub struct Anyone<'a> {
    pub state: &'a mut Manager,
    pub now: u64,
}

impl Tunnel for Anyone<'_> {
    fn open(&mut self, host: &[u8], port: u16) -> Result<u16, u8> {
        super::anyone_open::open(self.state, self.now, host, port)
    }

    fn far(&self, id: u16) -> Far {
        match self.state.streams.iter().find(|s| s.id == id).map(|s| s.stage) {
            Some(StreamStage::Opening) => Far::Opening,
            Some(StreamStage::Open) => Far::Open,
            Some(StreamStage::Ended(reason)) => Far::Ended(reason),
            None => Far::Gone,
        }
    }

    fn send(&mut self, id: u16, data: &[u8]) -> Result<usize, Unsent> {
        match send_data(self.state, id, data) {
            Ok(sent) => Ok(sent),
            Err(SendError::WouldBlock) => Err(Unsent::Blocked),
            Err(_) => Err(Unsent::Over),
        }
    }

    fn take(&mut self, id: u16, max: usize) -> Vec<u8> {
        super::anyone_stream::take(self.state, id, max)
    }

    fn waiting(&self, id: u16) -> usize {
        self.state.streams.iter().find(|s| s.id == id).map_or(0, |s| s.inbound.len())
    }

    fn close(&mut self, id: u16) {
        super::anyone_stream::close(self.state, id)
    }
}
