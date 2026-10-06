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

/* Which paths are models, and the volume name each one is. */

pub const ROOT: &[u8] = b"/models";
const NAME_MAX: usize = 63;

/* True for /models and every path below it. */
pub fn owns(path: &[u8]) -> bool {
    path.starts_with(ROOT) && matches!(path.get(ROOT.len()), None | Some(b'/'))
}

/*
 * The volume name for /models/<file>: "/<file>". None for /models itself,
 * for a deeper path, and for a name the volume would refuse.
 */
pub fn volume_name(path: &[u8]) -> Option<&[u8]> {
    let file = path.strip_prefix(ROOT)?.strip_prefix(b"/")?;
    let ok = |b: &u8| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b'-');
    let fits = !file.is_empty() && file.len() <= NAME_MAX && file[0] != b'.';
    (fits && file.iter().all(ok)).then(|| &path[ROOT.len()..])
}
