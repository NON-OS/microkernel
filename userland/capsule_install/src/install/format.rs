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

//! Numbers as a person reads them on this screen: sizes in the unit that
//! keeps three digits, rates per second, hashes as a short prefix.

use alloc::string::String;

/// "85.3 MB", "1.00 GB", "512 KB". Decimal units, as disk vendors label
/// them, so the number matches the sticker on the drive.
pub fn bytes(n: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut v = n as f64;
    let mut u = 0usize;
    while v >= 1000.0 && u < UNITS.len() - 1 {
        v /= 1000.0;
        u += 1;
    }
    if u == 0 {
        return alloc::format!("{n} B");
    }
    let tenths = (v * 10.0 + 0.5) as u64;
    alloc::format!("{}.{} {}", tenths / 10, tenths % 10, UNITS[u])
}

pub fn rate(bytes_done: u64, millis: u64) -> String {
    if millis == 0 {
        return String::from("...");
    }
    let per_second = bytes_done.saturating_mul(1000) / millis;
    alloc::format!("{}/s", bytes(per_second))
}

pub fn percent(done: u64, total: u64) -> u32 {
    if total == 0 {
        return 0;
    }
    (done.saturating_mul(100) / total).min(100) as u32
}

/// The first eight bytes of a digest as sixteen hex characters, which is
/// what the boot log prints and what a person compares against.
pub fn hex_prefix(digest: &[u8; 32]) -> String {
    let mut s = String::with_capacity(16);
    for b in &digest[..8] {
        s.push_str(&alloc::format!("{b:02x}"));
    }
    s
}
