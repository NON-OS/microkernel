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
//! `MkToolRun("tool.qwen", "window\0<tier>")`: the tier in its own desktop
//! window, the way the store starts it, rather than on the caller's
//! terminal. The run is queued for init like the store's, so it is attached
//! to no terminal and outlives the one that asked. Neither the tier word nor
//! the request is logged.

use super::super::embed::LINUX_ELF;
use super::super::roles::RUN;
use super::tier;
use crate::services::registry::lookup_service;
use crate::syscall::microkernel::errnos::{ERRNO_BUSY, ERRNO_INVAL, ERRNO_NOENT};

/// The leading word of a window request.
const WINDOW: &[u8] = b"window";

/// `window\0<tier>` (or a bare `window`, for the first tier) opens a window;
/// any other argument is the tier word of a run on the caller's terminal.
pub fn run_qwen_for_caller(argv: &[u8]) -> Result<u32, i64> {
    match window_word(argv) {
        Some(word) => open_window(word),
        None => super::run::run_tier_for_caller(argv),
    }
}

fn window_word(argv: &[u8]) -> Option<&[u8]> {
    match argv.strip_prefix(WINDOW)?.split_first() {
        None => Some(&[]),
        Some((0, word)) => Some(word),
        Some(_) => None,
    }
}

/*
 * Ok(0): the run is queued and init starts it on its next pass, so there is
 * no pid yet. EINVAL for a word outside the allowlist, before anything is
 * queued; ENOENT when this image carries no personality; EBUSY when the run
 * role is already live (a window or a store-started program holds it) or
 * the queue is full or already holds this run.
 */
fn open_window(word: &[u8]) -> Result<u32, i64> {
    let Some(index) = tier::parse(word) else {
        crate::sys::serial::print(b"[LINUX-WINDOW] refused: unknown tier\n");
        return Err(ERRNO_INVAL);
    };
    let package = tier::package(index).ok_or(ERRNO_INVAL)?;
    if LINUX_ELF.is_empty() {
        return Err(ERRNO_NOENT);
    }
    if lookup_service(RUN.name).is_some() {
        return Err(ERRNO_BUSY);
    }
    match crate::userspace::init::request_quiet_run(package) {
        true => Ok(0),
        false => Err(ERRNO_BUSY),
    }
}
