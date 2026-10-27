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

//! The `[LINUX-TIME]` lines an install ends with, pure so the host proofs
//! hold them. They count and time; they name no package, so a private run
//! says nothing here about what was installed.

use alloc::format;
use alloc::string::String;

/// Bytes as megabytes (10^6) to one decimal.
fn mb(bytes: u64) -> String {
    let tenths = bytes / 100_000;
    format!("{}.{}", tenths / 10, tenths % 10)
}

/// Milliseconds as seconds to one decimal.
fn secs(ms: u64) -> String {
    format!("{}.{}", ms / 1000, ms % 1000 / 100)
}

/// MB/s to two decimals, or `-` when no time passed.
fn rate(bytes: u64, ms: u64) -> String {
    match bytes.saturating_mul(100) / ms.saturating_mul(1000).max(1) {
        _ if ms == 0 => String::from("-"),
        hundredths => format!("{}.{:02}", hundredths / 100, hundredths % 100),
    }
}

/// One stage that moved bytes: `[LINUX-TIME] packages 25 gets 32.5 MB 98.0 s 0.33 MB/s`.
pub fn moved(stage: &str, gets: u64, bytes: u64, ms: u64) -> String {
    let (size, took, speed) = (mb(bytes), secs(ms), rate(bytes, ms));
    format!("[LINUX-TIME] {stage} {gets} gets {size} MB {took} s {speed} MB/s\n")
}

/// Where the rest went: `[LINUX-TIME] verify 3.2 s place 5.0 s total 112.3 s`.
pub fn spent(verify_ms: u64, place_ms: u64, total_ms: u64) -> String {
    let (v, p, t) = (secs(verify_ms), secs(place_ms), secs(total_ms));
    format!("[LINUX-TIME] verify {v} s place {p} s total {t} s\n")
}
