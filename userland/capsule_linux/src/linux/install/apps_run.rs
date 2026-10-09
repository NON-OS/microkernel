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

/// What a shipped tier starts as: the path its program was read from, the
/// program's bytes and its arguments.
pub type TierStart = (Vec<u8>, Vec<u8>, Vec<Vec<u8>>);

/// The shipped tier `name` as it starts in `mode`: None when `name` is no
/// shipped tier, and the store's reason when it is one but no build of its
/// program could be read.
pub fn launch(name: &str, mode: Mode) -> Option<Result<TierStart, &'static str>> {
    let app = app(name)?;
    let mut why = "no build of it is in the package store";
    for (program, said) in super::chat_build::choices(app.program) {
        let bytes = match store_read(&key(program), MAX_IMAGE) {
            Ok(bytes) => bytes,
            Err(e) => {
                why = e;
                continue;
            }
        };
        if !said.is_empty() {
            crate::linux::start::say(said);
        }
        return Some(Ok((program.to_vec(), bytes, mode.tier_args(app.args))));
    }
    Some(Err(why))
}
