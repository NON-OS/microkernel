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

//! What a delivered reply says before it is filed: its size, and for a
//! connection error the class of the exit's refusal (tunnel/refusal.rs),
//! never its message, which names the host.

use nonos_libc::mk_debug;

pub fn reply(msg: &[u8]) {
    super::trace::reply_bytes(msg.len());
    let Some(class) = crate::tunnel::refusal(msg) else {
        return;
    };
    let mut line = [0u8; 48];
    let head = b"[SOCKS5] exit refused: ";
    let word = class.word();
    line[..head.len()].copy_from_slice(head);
    line[head.len()..head.len() + word.len()].copy_from_slice(word);
    let n = head.len() + word.len();
    line[n] = b'\n';
    mk_debug(line.as_ptr(), n + 1);
}
