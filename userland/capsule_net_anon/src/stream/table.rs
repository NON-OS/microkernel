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

//! The open streams on a circuit, and the ids they hold.

extern crate alloc;

use alloc::vec::Vec;

use crate::circuit::window::{STREAM_INCREMENT, STREAM_START};

use super::stage::StreamStage;

pub struct Stream {
    pub id: u16,
    pub stage: StreamStage,
    /// Which circuit carries it, by circuit id.
    pub circuit: u32,
    /// The pid that opened it, or `owned::FRONT` for the SOCKS front.
    pub owner: u32,
    /// Bytes the far end has not yet been asked to read.
    pub inbound: Vec<u8>,
    /// Cells we may still send before the far end grants more.
    pub package_window: i32,
    /// Cells that may still arrive before we owe a SENDME.
    pub deliver_window: i32,
    /// Cells delivered since the last SENDME on this stream.
    pub delivered_since: i32,
}

impl Stream {
    pub fn new(id: u16, circuit: u32) -> Self {
        Self {
            id,
            stage: StreamStage::Opening,
            circuit,
            owner: 0,
            inbound: Vec::new(),
            package_window: STREAM_START,
            deliver_window: STREAM_START,
            delivered_since: 0,
        }
    }

    /// The far end granted this stream another STREAM_INCREMENT cells.
    pub fn credit_package(&mut self) {
        self.package_window = self.package_window.saturating_add(STREAM_INCREMENT);
    }

    /// End a stream whose circuit is gone, unless it has already ended.
    pub fn end_with_circuit(&mut self) {
        if !matches!(self.stage, StreamStage::Ended(_)) {
            self.stage = StreamStage::Ended(REASON_DESTROY);
        }
    }
}

/// The END reason a stream gets when its circuit is torn down (tor-spec 6.3).
pub const REASON_DESTROY: u8 = 5;
