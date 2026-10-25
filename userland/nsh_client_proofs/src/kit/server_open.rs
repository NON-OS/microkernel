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

//! Opening what the client sent, frame by frame, with the server's own
//! count.

use super::server::Server;
use nsh_wire::TAG_LEN;

impl Server {
    /// Open every frame the client sent, as (type, body).
    pub fn open(&mut self, mut wire: &[u8]) -> Option<Vec<(u8, Vec<u8>)>> {
        let mut frames = Vec::new();
        while wire.len() >= 4 {
            let len: [u8; 4] = wire[..4].try_into().unwrap();
            let n = u32::from_le_bytes(len) as usize;
            let sealed = wire.get(4..4 + n)?;
            let mut inner = vec![0u8; n.saturating_sub(TAG_LEN)];
            self.opener.as_mut()?.open(len, sealed, &mut inner).ok()?;
            frames.push((inner[0], inner[1..].to_vec()));
            wire = &wire[4 + n..];
        }
        wire.is_empty().then_some(frames)
    }
}
