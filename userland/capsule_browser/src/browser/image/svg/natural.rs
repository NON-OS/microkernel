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

use super::viewport::viewport;

pub fn is_svg(bytes: &[u8]) -> bool {
    let head = &bytes[..bytes.len().min(1024)];
    core::str::from_utf8(head).map(|s| s.contains("<svg")).unwrap_or(false)
        || head.windows(4).any(|w| w == b"<svg")
}

/// The document's natural size as an `<img>` reports it (see viewport),
/// each side rounded to the nearest whole pixel, at least 1.
pub fn natural_size(bytes: &[u8]) -> Option<(u32, u32)> {
    let doc = core::str::from_utf8(bytes).ok()?;
    let v = viewport(doc)?;
    Some((round(v.natural.0), round(v.natural.1)))
}

fn round(v: f32) -> u32 {
    ((v + 0.5) as u32).max(1)
}
