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

//! One command as the card layer asks for it, with its data phase.

use super::super::super::env::DmaBuf;
use super::super::cmd::Resp;

/// The data phase of a command.
#[derive(Debug, Clone, Copy)]
pub struct Data {
    pub read: bool,
    pub blocks: u16,
    pub multi: bool,
    pub auto12: bool,
    pub buf: DmaBuf,
}

#[derive(Debug, Clone, Copy)]
pub struct Cmd {
    pub index: u8,
    pub arg: u32,
    pub resp: Resp,
    pub data: Option<Data>,
    /// CMD12 ending a transfer: Command Type Abort, DAT inhibit not awaited.
    pub abort: bool,
    /// How long the data or busy phase may take.
    pub wait_ms: u64,
}

impl Cmd {
    pub const fn new(index: u8, arg: u32, resp: Resp) -> Self {
        Self { index, arg, resp, data: None, abort: false, wait_ms: 1_000 }
    }
}
