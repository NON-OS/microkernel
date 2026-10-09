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
 * The names the import path keeps beside a file it brought in: the record
 * of the digest it was verified against, `<name>.sha256`, and the mark of
 * a stream still coming, `<name>.partial`. Only the import path makes,
 * changes or removes one. Pure, so the host tests the very same source.
 */

use alloc::vec::Vec;

const RECORD: &[u8] = b".sha256";
const MARK: &[u8] = b".partial";

/* `path` without the slashes after it: the name the volume resolves. */
pub(super) fn trimmed(path: &[u8]) -> &[u8] {
    let end = path.iter().rposition(|&b| b != b'/').map_or(0, |i| i + 1);
    &path[..end]
}

/* Whether `path` names a record or a mark. */
pub(in super::super) fn is_kept(path: &[u8]) -> bool {
    let name = trimmed(path);
    name.ends_with(RECORD) || name.ends_with(MARK)
}

/* Where the record for the file at `path` would be. */
pub(super) fn record_of(path: &[u8]) -> Vec<u8> {
    [trimmed(path), RECORD].concat()
}

/* Where the mark of a stream coming to `path` would be. */
pub(super) fn mark_of(path: &[u8]) -> Vec<u8> {
    [trimmed(path), MARK].concat()
}
