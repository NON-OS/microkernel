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

/* The commands and lock types fcntl's record locks take. */

pub const F_GETLK: u64 = 5;

pub const F_SETLK: u64 = 6;

pub const F_SETLKW: u64 = 7;

pub const F_OFD_GETLK: u64 = 36;

pub const F_OFD_SETLK: u64 = 37;

pub const F_OFD_SETLKW: u64 = 38;

pub(super) const F_RDLCK: i16 = 0;

pub(super) const F_WRLCK: i16 = 1;

pub(super) const F_UNLCK: i16 = 2;

pub(super) const FLOCK: usize = 32;

pub fn is_lock(cmd: u64) -> bool {
    matches!(cmd, F_GETLK | F_SETLK | F_SETLKW | F_OFD_GETLK | F_OFD_SETLK | F_OFD_SETLKW)
}
