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

//! A plain `linux` run whose program's policy grants NET_ANON: the
//! personality asks the kernel for a ticket and exits with LINUX_ASK_NET,
//! and the terminal starts the same argv once more as `tool.linux.net`.
//! Pure, so a host proof holds it as it ships (net_again.rs acts on it).

use alloc::vec::Vec;

/// The service such a run is started again as, and the one it starts as.
pub const NET_TOOL: &[u8] = b"tool.linux.net";
pub const PLAIN_TOOL: &[u8] = b"tool.linux";

/// The run that may ask: its pid, and what starts it again on this screen.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetAsk {
    pub pid: u32,
    pub argv: Vec<u8>,
    pub streams: u64,
}

impl NetAsk {
    /// Only a plain `tool.linux` run may ask; a `.net` one never asks again.
    pub fn of(service: &[u8], pid: u32, argv: &[u8], streams: u64) -> Option<Self> {
        (service == PLAIN_TOOL).then(|| Self { pid, argv: argv.to_vec(), streams })
    }
}

/// `pid` ended with `status`: what to start again, if it was the run that
/// may ask and it asked. The run's chance is spent when it ends, whatever
/// it ended with, so it is started again at most once.
pub fn ended(held: &mut Option<NetAsk>, pid: u32, status: i32, ask_net: i32) -> Option<NetAsk> {
    if held.as_ref().is_none_or(|n| n.pid != pid) {
        return None;
    }
    held.take().filter(|_| status == ask_net)
}
