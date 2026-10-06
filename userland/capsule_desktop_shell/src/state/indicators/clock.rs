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

//! The menubar's date and time line, from a moment already read: "Sat 03 Oct
//! 19:42" on a 24 hour clock, "Sat 03 Oct  7:42 PM" on a 12 hour one.
//!
//! The 12 hour clock used to drop the 24 hour hour into twelve and show no
//! AM or PM, so 07:42 in the morning and the evening read the same.

/// The longest line: weekday, day, month, two spaces, the time and " AM".
pub const LINE_MAX: usize = 20;

/// A local date and time, as the clock reads it.
#[derive(Clone, Copy)]
pub struct Moment {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
}

const DAYS: [&[u8; 3]; 7] = [b"Sun", b"Mon", b"Tue", b"Wed", b"Thu", b"Fri", b"Sat"];
const MONTHS: [&[u8; 3]; 12] = [
    b"Jan", b"Feb", b"Mar", b"Apr", b"May", b"Jun", b"Jul", b"Aug", b"Sep", b"Oct", b"Nov", b"Dec",
];
const SHIFT: [i32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];

/// Write the line for `m` into `buf` and give its length.
pub fn line(buf: &mut [u8; LINE_MAX], m: &Moment, h24: bool) -> usize {
    let month = (m.month as usize).clamp(1, 12);
    buf[..3].copy_from_slice(DAYS[weekday(m.year as i32, month, m.day as i32)]);
    buf[3] = b' ';
    buf[4] = b'0' + (m.day / 10) % 10;
    buf[5] = b'0' + m.day % 10;
    buf[6] = b' ';
    buf[7..10].copy_from_slice(MONTHS[month - 1]);
    buf[10] = b' ';
    buf[11] = b' ';
    let hour = if h24 {
        m.hour
    } else {
        match m.hour % 12 {
            0 => 12,
            h => h,
        }
    };
    // A 12 hour clock shows " 7:42", as clocks do; a 24 hour one "07:42".
    buf[12] = if !h24 && hour < 10 { b' ' } else { b'0' + (hour / 10) % 10 };
    buf[13] = b'0' + hour % 10;
    buf[14] = b':';
    buf[15] = b'0' + (m.minute / 10) % 10;
    buf[16] = b'0' + m.minute % 10;
    if h24 {
        return 17;
    }
    buf[17] = b' ';
    buf[18] = if m.hour < 12 { b'A' } else { b'P' };
    buf[19] = b'M';
    LINE_MAX
}

fn weekday(year: i32, month: usize, day: i32) -> usize {
    let y = year - (month < 3) as i32;
    (y + y / 4 - y / 100 + y / 400 + SHIFT[month - 1] + day).rem_euclid(7) as usize
}
