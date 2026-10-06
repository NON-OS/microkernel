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
 * The kernel's side of a download: a stream into the data volume, begun
 * for one pinned file, fed chunk by chunk, and finished only when every
 * byte came and their SHA-256 is the pin. Each refusal is the errno.
 */

use nonos_libc::data::{mk_data_feed, mk_data_feed_begin, mk_data_feed_end};

const EALREADY: i64 = -114;

/* Where a file's stream stands. */
pub enum Start {
    /* Imported and verified before. */
    Done,
    /* Feed on from this byte. */
    From(u64),
}

/*
 * Begin, or take up, the stream for `name` (with its slash); with `probe`,
 * only ask where it stands.
 */
pub fn begin(name: &[u8], sha256: &[u8; 32], bytes: u64, probe: bool) -> Result<Start, i64> {
    match mk_data_feed_begin(name, sha256, bytes, probe) {
        EALREADY => Ok(Start::Done),
        at if at >= 0 => Ok(Start::From(at as u64)),
        e => Err(e),
    }
}

/* Feed `chunk`; how far the stream has come. */
pub fn feed(chunk: &[u8]) -> Result<u64, i64> {
    let at = mk_data_feed(chunk);
    if at < 0 {
        Err(at)
    } else {
        Ok(at as u64)
    }
}

/* Finish, the kernel checking the SHA-256; the file's size. */
pub fn finish() -> Result<u64, i64> {
    let n = mk_data_feed_end(false);
    if n < 0 {
        Err(n)
    } else {
        Ok(n as u64)
    }
}

/* Put the stream down with its mark saved, for a later `qwen get`; where it stands. */
pub fn pause() -> Result<u64, i64> {
    let n = mk_data_feed_end(true);
    if n < 0 {
        Err(n)
    } else {
        Ok(n as u64)
    }
}
