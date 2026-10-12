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

//! The capsule's lines on the debug channel, each prefixed `bt-usb:`.

use alloc::vec::Vec;

use nonos_bt_hci::HciError;

pub fn say(words: &[&[u8]]) {
    let mut line: Vec<u8> = b"bt-usb: ".to_vec();
    for w in words {
        line.extend_from_slice(w);
    }
    line.push(b'\n');
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
}

/// `n` in decimal, at least `width` digits.
pub fn digits(mut n: u64, width: usize) -> Vec<u8> {
    let mut out = Vec::new();
    while n > 0 || out.len() < width.max(1) {
        out.push(b'0' + (n % 10) as u8);
        n /= 10;
    }
    out.reverse();
    out
}

/// The words for the error that ended a controller's run.
pub fn reason(e: &HciError) -> &'static [u8] {
    match e {
        HciError::Transport(_) => b"USB transfer failed",
        HciError::Timeout(_) => b"controller stopped answering",
        HciError::Status(..) => b"controller refused a command",
        HciError::NoLe | HciError::NotBluetooth => b"not an LE controller this host can drive",
        HciError::Vendor => b"its vendor's setup was not done",
        _ => b"controller sent something malformed",
    }
}
