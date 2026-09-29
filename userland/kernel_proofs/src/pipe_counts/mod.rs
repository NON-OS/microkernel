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
 * A pipe's reader and writer counts must stop at zero.
 *
 * The kernel's PipeBuffer is included by path. remove_reader and remove_writer
 * were fetch_sub, and PipeReader::close followed by its drop called them twice
 * for one endpoint, taking the count past zero to usize::MAX, after which the
 * pipe reported a reader or writer forever. The check below fails against
 * that code.
 */

#[path = "../../../../src/fs/pipe/buffer.rs"]
pub mod buffer;
mod tests;
