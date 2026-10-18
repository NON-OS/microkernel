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

//! What one sample holds.

use super::super::census::Census;
use super::super::session::Boot;
use alloc::vec::Vec;
use nonos_libc::PROC_NAME_LEN;
use nonos_route_proof::{Latest, RouteVerdict};

/// An admitted capsule that can reach the network.
#[derive(Clone, Copy)]
pub struct Holder {
    pub pid: u32,
    pub name: [u8; PROC_NAME_LEN],
    pub name_len: usize,
    pub measurement: [u8; 32],
    pub authority: u8,
}

pub struct Snapshot {
    pub boot: Boot,
    /// `None` when the kernel would not show the table or the registry.
    pub census: Option<Census>,
    pub holders: Vec<Holder>,
    /// The policy store's route, `None` when it did not answer.
    pub chosen: Option<u8>,
    /// The board's answer, `None` when the attest service did not answer.
    pub board: Option<(Latest, Latest)>,
    pub route: RouteVerdict,
}

pub(super) struct Row {
    pub(super) pid: u32,
    pub(super) state: u8,
    pub(super) uptime_ms: u64,
    pub(super) caps: u64,
    pub(super) name: [u8; PROC_NAME_LEN],
    pub(super) len: usize,
}
