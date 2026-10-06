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

/// Why a response could not be read, with the numbers that say so.
///
/// "bad response" alone told a reader nothing, and it covered two different
/// things: a body that stopped short of what its headers promised, and bytes
/// that were never an HTTP response at all.
pub(super) fn incomplete(raw: &[u8]) -> alloc::string::String {
    let Some(sep) = raw.windows(4).position(|w| w == b"\r\n\r\n") else {
        return alloc::format!(
            "The answer was cut short after {} bytes, before its headers ended. Reload to try \
             again.",
            raw.len()
        );
    };
    let body = raw.len() - sep - 4;
    let head = core::str::from_utf8(&raw[..sep]).unwrap_or("");
    let declared = head.lines().find_map(|l| {
        let (k, v) = l.split_once(':')?;
        k.trim().eq_ignore_ascii_case("content-length").then(|| v.trim())
    });
    match declared {
        Some(n) => alloc::format!(
            "The page was cut short: {} of the {} bytes it declared arrived. Reload to try again.",
            body,
            n
        ),
        None => alloc::format!(
            "The page was cut short after {} bytes, before its end. Reload to try again.",
            body
        ),
    }
}
