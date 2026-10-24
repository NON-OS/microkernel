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

//! A boxed run's report, on its way to the terminal that started it. The
//! terminal names a key only it and this capsule know (`--box-key=<hex>`,
//! never in the guest's argv), and every frame of the report opens with
//! TAG_BOX and that key: the guest writes the same channel, but cannot write
//! a frame the terminal takes for the report.

use alloc::vec::Vec;

/// nonos_libc::TAG_BOX; the capsule holds the two equal at build time.
pub const TAG: u8 = 3;
pub const KEY_LEN: usize = 16;
pub const KEY_ARG: &[u8] = b"--box-key=";
/// A message the terminal reads at once (mk_proc_output's 256 bytes).
pub const FRAME_MAX: usize = 256;

/// The key `word` names, if it is `--box-key=` and 32 lowercase hex digits.
pub fn key_of(word: &[u8]) -> Option<[u8; KEY_LEN]> {
    let hex = word.strip_prefix(KEY_ARG)?;
    if hex.len() != 2 * KEY_LEN {
        return None;
    }
    let mut key = [0u8; KEY_LEN];
    for (k, pair) in key.iter_mut().zip(hex.chunks(2)) {
        *k = (digit(pair[0])? << 4) | digit(pair[1])?;
    }
    Some(key)
}

fn digit(d: u8) -> Option<u8> {
    match d {
        b'0'..=b'9' => Some(d - b'0'),
        b'a'..=b'f' => Some(d - b'a' + 10),
        _ => None,
    }
}

/// `text` as the frames that carry it, each at most FRAME_MAX bytes.
pub fn frames(key: &[u8; KEY_LEN], text: &[u8]) -> Vec<Vec<u8>> {
    text.chunks(FRAME_MAX - 1 - KEY_LEN).map(|piece| [&[TAG][..], key, piece].concat()).collect()
}
