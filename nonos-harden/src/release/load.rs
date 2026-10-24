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

//! `release.record`: the directory holds a release record, in the format this
//! tool reads. Every other check over the release starts from it, and the
//! verdict names the record by its SHA-256, so a verdict is tied to bytes.

use super::read::read_regular;
use super::record::{parse, FORMAT, MAX_BYTES};
use super::RECORD;
use crate::check::Check;
use crate::ctx::Ctx;
use crate::hash::sha256_hex;

pub const ID: &str = "release.record";

pub fn check(ctx: &mut Ctx) -> Check {
    let bytes = match read_regular(&ctx.release.join(RECORD), MAX_BYTES) {
        Ok(b) => b,
        Err(e) => return Check::fail(ID, format!("{RECORD}: {e}")),
    };
    ctx.record_sha256 = Some(sha256_hex(&bytes));
    match parse(&bytes) {
        Ok(r) => {
            let n = r.sealed.len();
            ctx.record = Some(r);
            Check::pass(ID, format!("{FORMAT}, naming {n} sealed artifacts"))
        }
        Err(e) => Check::fail(ID, format!("{RECORD}: {e}")),
    }
}
