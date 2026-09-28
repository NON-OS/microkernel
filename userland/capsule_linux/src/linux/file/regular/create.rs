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

/* A file made by O_CREAT, with the mode it was asked for. */

use alloc::vec::Vec;

use crate::linux::abi::errno;
use crate::linux::guest::{Fd, Guest};

use super::super::flags::writes;
use super::super::{cache, resolve};
use super::open::install;

/*
 * Linux makes the file at open, so stat sees it before anything is written;
 * here it is held empty in the family's copy until close puts it in the store.
 */
pub fn create(guest: &mut Guest, path: Vec<u8>, flags: u64) -> u64 {
    if resolve::key(&path).writable().is_err() {
        return errno::fail(errno::EROFS);
    }
    if let Err(e) = cache::hold(&path, false) {
        return errno::fail(e);
    }
    install(guest, Fd::file(path, 0, None, writes(flags)), flags)
}
