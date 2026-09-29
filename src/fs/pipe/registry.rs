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

extern crate alloc;

use super::reader::PipeReader;
use super::writer::PipeWriter;
use alloc::collections::BTreeMap;
use spin::Mutex;

/* An open pipe descriptor owns its endpoint. Dropping the endpoint when the
descriptor is unregistered is the one place its count goes down. */
pub enum PipeEnd {
    Reader { _end: PipeReader },
    Writer { _end: PipeWriter },
}

static PIPE_ENDS: Mutex<BTreeMap<i32, PipeEnd>> = Mutex::new(BTreeMap::new());

pub fn register_pipe_end(fd: i32, end: PipeEnd) {
    let replaced = PIPE_ENDS.lock().insert(fd, end);
    drop(replaced);
}

pub fn is_pipe_end(fd: i32) -> bool {
    PIPE_ENDS.lock().contains_key(&fd)
}

pub fn unregister_pipe_end(fd: i32) {
    let end = PIPE_ENDS.lock().remove(&fd);
    drop(end);
}
