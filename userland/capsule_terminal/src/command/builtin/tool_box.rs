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

//! A `linux --box` run's key, drawn here and named to the run ahead of its
//! words (jobs/box_hold.rs, tool_argv.rs).

use crate::jobs::box_hold::KEY_LEN;

/// The key a boxed run's report is sent under; None for any other run.
pub fn boxed(args: &[&[u8]]) -> Result<Option<[u8; KEY_LEN]>, &'static [u8]> {
    if args.first() != Some(&&b"linux"[..]) || args.get(1) != Some(&&b"--box"[..]) {
        return Ok(None);
    }
    let mut key = [0u8; KEY_LEN];
    match nonos_libc::crypto_random(key.as_mut_ptr(), KEY_LEN) {
        n if n < 0 => Err(b"linux: --box needs a random key, and none could be drawn"),
        _ => Ok(Some(key)),
    }
}
