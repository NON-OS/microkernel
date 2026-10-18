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

//! The NVGP display wire, as the compositor's gfx client speaks it to
//! virtio-gpu (userland/compositor/src/gfx_client/wire.rs): a 20-byte
//! header, every field little-endian, then the body. A reply carries the
//! request's header, a 4-byte status and, for get_primary, its body.

pub const MAGIC: u32 = 0x4E56_4750;
pub const VERSION: u16 = 1;
pub const HDR_LEN: usize = 20;
pub const STATUS_LEN: usize = 4;
/// The receive buffer. Every request served is far shorter, so one longer,
/// which the kernel cuts to this, is refused for its length, never read cut.
pub const RX_LEN: usize = HDR_LEN + 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Request {
    pub op: u16,
    pub flags: u16,
    pub request_id: u32,
}
