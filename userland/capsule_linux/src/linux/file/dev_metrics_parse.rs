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
 * One pair a guest reports: a name from a fixed list, and a decimal integer.
 * Nothing else parses, so nothing else is ever printed.
 */

pub const NAMES: [&str; 12] = [
    "match",
    "tokens",
    "prompt_tokens",
    "threads",
    "load_ms",
    "ttft_ms",
    "decode_ms",
    "tok_per_s_x100",
    "peak_rss_kb",
    "model_bytes",
    "fail_stage",
    "errno",
];
pub const MAX_LINE: u64 = 512;
const MAX_VALUE: u64 = 1_000_000_000_000_000;

/* One `name=value`: a listed name and a decimal integer, or None. */
pub fn parse(pair: &[u8]) -> Option<(&'static str, u64)> {
    let eq = pair.iter().position(|b| *b == b'=')?;
    let name = NAMES.iter().find(|n| n.as_bytes() == &pair[..eq])?;
    let digits = &pair[eq + 1..];
    if digits.is_empty() || digits.len() > 16 || !digits.iter().all(u8::is_ascii_digit) {
        return None;
    }
    let value = digits.iter().fold(0u64, |v, d| v * 10 + u64::from(d - b'0'));
    (value <= MAX_VALUE).then_some((*name, value))
}
