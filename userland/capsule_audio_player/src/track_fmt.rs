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

extern crate alloc;
use alloc::string::String;

use crate::decode::Decoder;

/// The Now Playing label: the format the decoder found and, when the file
/// gave one, its sample rate ("MP3 44100Hz"). Every track read "WAV" before,
/// MP3s included.
pub fn format_of(dec: &dyn Decoder) -> String {
    let info = dec.info();
    let mut s = String::from(dec.kind());
    if info.rate > 0 {
        s.push(' ');
        push_u32(&mut s, info.rate);
        s.push_str("Hz");
    }
    s
}

fn push_u32(s: &mut String, mut n: u32) {
    if n == 0 {
        s.push('0');
        return;
    }
    let mut tmp = [0u8; 10];
    let mut i = 0;
    while n > 0 {
        tmp[i] = b'0' + (n % 10) as u8;
        n /= 10;
        i += 1;
    }
    while i > 0 {
        i -= 1;
        s.push(tmp[i] as char);
    }
}
