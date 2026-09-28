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
 * The positional forms: pread64, pwrite64, preadv, pwritev, and preadv2 and
 * pwritev2 with no flags. Each reads or writes at the offset it is given
 * and leaves the descriptor's own offset where it was; a pipe, a socket or
 * a console has no offset, which Linux calls ESPIPE.
 */

mod plain;
mod sync;
mod vector;

pub use plain::{pread64, pwrite64};
pub use sync::{preadv, pwritev};
