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

/* The first line of `b`: where its content ends and where the next line
starts. A line ends at LF, a CR just before it dropping out of the
content (RFC 9112 2.2 lets a recipient take a bare LF as a line end).
None until the LF has arrived. */
pub fn line_end(b: &[u8]) -> Option<(usize, usize)> {
    let lf = b.iter().position(|&c| c == b'\n')?;
    let end = if lf > 0 && b[lf - 1] == b'\r' { lf - 1 } else { lf };
    Some((end, lf + 1))
}
