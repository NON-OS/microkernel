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

/* Reading and writing a made file. */

use alloc::vec::Vec;

use crate::linux::abi::errno;

use super::super::super::{proc, sys};
use super::super::synth::{self};

/* The bytes at `offset` of the made file at `path`; None if it is not one. */
pub fn read(path: &[u8], offset: u64, len: usize) -> Option<Result<Vec<u8>, i64>> {
    if !synth::owns(path) {
        return None;
    }
    let whole = match sys::content(path) {
        Some(bytes) => Ok(bytes),
        None => proc::content(path),
    };
    Some(whole.map(|all| {
        let from = (offset as usize).min(all.len());
        all[from..(from + len).min(all.len())].to_vec()
    }))
}

/* A write to the made file at `path`, which none of them takes. */
pub fn write(path: &[u8]) -> Option<Result<usize, i64>> {
    if !synth::owns(path) {
        return None;
    }
    /* The devices are answered by file/dev.rs; nothing made here takes a write. */
    Some(Err(errno::EACCES))
}
