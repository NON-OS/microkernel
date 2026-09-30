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

//! What a shipped tier starts as: its program from the store, and its
//! arguments for the way it was asked to run.

use alloc::vec::Vec;

use super::apps::app;
use crate::linux::file::{key, store_read};
use crate::linux::run_mode::Mode;

const MAX_IMAGE: u32 = 64 << 20;

/// The shipped tier `name` starts as in `mode`, or None if it names none.
pub fn launch(name: &str, mode: Mode) -> Option<(Vec<u8>, Vec<u8>, Vec<Vec<u8>>)> {
    let app = app(name)?;
    let program = super::isa::for_this_cpu(app.program);
    if program != app.program {
        crate::linux::start::say(b"[LINUX] no AVX2 on this CPU: running the x86-64-v2 build\n");
    }
    let bytes = store_read(&key(program), MAX_IMAGE).ok()?;
    Some((program.to_vec(), bytes, mode.tier_args(app.args)))
}
