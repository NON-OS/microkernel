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

//! The numbers both sides use, and the bodies of READ and WRITE.

pub const MAGIC: [u8; 4] = *b"NSO1";
pub const HEADER: usize = 12;
/// The most bytes a frame carries after its header.
pub const MAX_BODY: usize = 64 * 1024;
/// The longest a READ waits for the guest to write.
pub const MAX_WAIT_MS: u32 = 5_000;
/// The endpoint a served box is reached at, followed by the capsule's name.
pub const ENDPOINT_PREFIX: &str = "linux.serve.";

pub const OPEN: u8 = 1;
pub const WRITE: u8 = 2;
pub const READ: u8 = 3;
pub const CLOSE: u8 = 4;

pub const OK: u8 = 0;
pub const CLOSED: u8 = 1;
pub const EBADF: u8 = 9;
pub const EAGAIN: u8 = 11;
pub const EBUSY: u8 = 16;
pub const EINVAL: u8 = 22;
pub const ECONNREFUSED: u8 = 111;

/// READ's question body: how much the asker takes, and how long it waits.
pub fn read_ask(want: u32, wait_ms: u32) -> [u8; 8] {
    let mut out = [0u8; 8];
    out[..4].copy_from_slice(&want.to_le_bytes());
    out[4..].copy_from_slice(&wait_ms.to_le_bytes());
    out
}

/// A READ question's body read back, refused past the bounds.
pub fn read_asked(body: &[u8]) -> Option<(u32, u32)> {
    let want = u32::from_le_bytes(body.get(..4)?.try_into().ok()?);
    let wait = u32::from_le_bytes(body.get(4..8)?.try_into().ok()?);
    let fits = body.len() == 8 && want > 0 && want as usize <= MAX_BODY;
    (fits && wait <= MAX_WAIT_MS).then_some((want, wait))
}

/// WRITE's answer body: how many bytes the guest's buffer took.
pub fn took(n: u32) -> [u8; 4] {
    n.to_le_bytes()
}

/// A WRITE answer's body read back.
pub fn took_of(body: &[u8]) -> Option<u32> {
    (body.len() == 4).then(|| u32::from_le_bytes([body[0], body[1], body[2], body[3]]))
}
