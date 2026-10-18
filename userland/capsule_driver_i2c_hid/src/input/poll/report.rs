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

//! The report in a frame read from the input register: what its two-byte
//! length prefix covers after itself.

pub(super) enum Report<'a> {
    Whole(&'a [u8]),
    /// The prefix is shorter than itself, or longer than what was read.
    Torn,
}

/// The report in the first `n` bytes of `frame`.
pub(super) fn report_of(frame: &[u8], n: usize) -> Report<'_> {
    let Some(&[lo, hi]) = frame.first_chunk::<2>() else { return Report::Torn };
    let total = usize::from(u16::from_le_bytes([lo, hi]));
    match frame.get(2..total) {
        Some(report) if total <= n => Report::Whole(report),
        _ => Report::Torn,
    }
}
