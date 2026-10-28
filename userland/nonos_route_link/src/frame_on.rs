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

/*
 * A frame that names its stream. Both proxies key a conversation on the
 * caller's pid and the stream a frame names, a frame naming none being
 * stream 0 (capsule_socks5 server/who.rs, capsule_net_anon
 * server/socks/who.rs), so a caller that holds several streams at once
 * names each: the Linux personality, one caller for every guest it hosts.
 * The tunnel builds its frames for stream 0 (`frame`), and the carrier
 * names the stream on the way out, so the opening, the slices and the
 * numbering are the code every other caller runs.
 */

use alloc::vec::Vec;

use crate::frame::{NUMBERED, RESET};

/* A reset naming its stream: the marker, then the stream, u32 little endian. */
pub const RESET_ON: u8 = 3;

/* A numbered exchange naming its stream: the marker, the stream, the number, the bytes. */
pub const NUMBERED_ON: u8 = 4;

/* The most streams one caller holds at either proxy, stream 0 among them. */
pub const STREAMS_PER_CALLER: u32 = 8;

/*
 * `frame`, as stream `stream` sends it: None for stream 0, whose frames go
 * as they are built, and for a frame that is neither a reset nor a numbered
 * exchange, which no tunnel builds.
 */
pub fn on(stream: u32, frame: &[u8]) -> Option<Vec<u8>> {
    if stream == 0 {
        return None;
    }
    let (marker, rest) = match frame.split_first()? {
        (&RESET, _) => (RESET_ON, &[][..]),
        (&NUMBERED, rest) => (NUMBERED_ON, rest),
        _ => return None,
    };
    let mut out = Vec::with_capacity(5 + rest.len());
    out.push(marker);
    out.extend_from_slice(&stream.to_le_bytes());
    out.extend_from_slice(rest);
    Some(out)
}
