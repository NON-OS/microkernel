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

//! The label for a reading, never a number the kernel did not give.

use super::reading::Battery;

/// The longest label, "Battery status unavailable".
pub const LABEL_MAX: usize = 26;

pub const NO_BATTERY: &[u8] = b"No battery";

pub const UNAVAILABLE: &[u8] = b"Battery status unavailable";

/// Said after a reading firmware says is charging.
pub const CHARGING: &[u8] = b" Charging";

fn put(buf: &mut [u8; LABEL_MAX], at: usize, words: &[u8]) -> usize {
    buf[at..at + words.len()].copy_from_slice(words);
    at + words.len()
}

/// "57%", or "57% Charging", for a reading; "No battery" or "Battery status
/// unavailable" otherwise: never a number the kernel did not give.
pub fn label(b: Battery, buf: &mut [u8; LABEL_MAX]) -> usize {
    let p = match b {
        Battery::None => return put(buf, 0, NO_BATTERY),
        Battery::Unavailable => return put(buf, 0, UNAVAILABLE),
        Battery::Percent(p) | Battery::Charging(p) => p.min(100),
    };
    let mut n = 0;
    if p >= 100 {
        n = put(buf, 0, b"100");
    } else {
        if p >= 10 {
            buf[n] = b'0' + (p / 10) as u8;
            n += 1;
        }
        buf[n] = b'0' + (p % 10) as u8;
        n += 1;
    }
    n = put(buf, n, b"%");
    match b {
        Battery::Charging(_) => put(buf, n, CHARGING),
        _ => n,
    }
}
