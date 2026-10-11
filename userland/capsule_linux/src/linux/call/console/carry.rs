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

//! A console write carried out of the capsule, a line at a time.

use crate::linux::abi::errno;
use crate::linux::console::{private, split, wipe};

/// The longest line the kernel's debug channel and a private write take;
/// each refuses a longer one whole (src/syscall/microkernel/debug.rs and
/// private_write.rs MAX_LEN).
const MAX_LINE: usize = 256;

/*
 * Forwarded in pieces the kernel takes. The count returned is what was
 * carried: a write refused part way is a short write, as Linux reports one,
 * never a claimed success. A launcher's inbox that is full takes nothing,
 * and that is EAGAIN, which parks a blocking writer until it drains.
 */
/// `bytes` to the console from `stream` (1 stdout, 2 stderr), as `write`
/// answers. On a split console each message leads with its stream's tag.
pub fn carry(bytes: &[u8], stream: u8) -> u64 {
    let private = private();
    let tag = match (split(), stream) {
        (false, _) => None,
        (true, 2) => Some(nonos_libc::TAG_STDERR),
        (true, _) => Some(nonos_libc::TAG_STDOUT),
    };
    let mut done = 0;
    let mut last = 0;
    let mut framed = [0u8; MAX_LINE];
    for piece in bytes.chunks(MAX_LINE - usize::from(tag.is_some())) {
        let msg: &[u8] = match tag {
            Some(t) => {
                framed[0] = t;
                framed[1..=piece.len()].copy_from_slice(piece);
                &framed[..=piece.len()]
            }
            None => piece,
        };
        last = match private {
            true => nonos_libc::mk_private_write(msg),
            false => nonos_libc::mk_debug(msg.as_ptr(), msg.len()),
        };
        if last < 0 {
            break;
        }
        done += piece.len();
    }
    wipe(&mut framed);
    match done {
        0 if last == -errno::EBUSY && private => errno::fail(errno::EAGAIN),
        0 => errno::fail(errno::EIO),
        n => errno::ok(n as u64),
    }
}
