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

use super::record::{parse, FORMAT, MAX_BYTES};
use super::RECORD;
use crate::check::Check;
use crate::ctx::Ctx;
use crate::hash::sha256_hex;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

pub const ID: &str = "release.record";

pub fn check(ctx: &mut Ctx) -> Check {
    let bytes = match read(&ctx.release.join(RECORD)) {
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

/// The record's bytes, when it is a regular file; read no further than one
/// byte past the largest record, which `parse` then refuses.
fn read(at: &Path) -> Result<Vec<u8>, String> {
    let md = fs::symlink_metadata(at).map_err(|_| "not found; this is not a sealed release")?;
    if !md.file_type().is_file() {
        return Err("not a regular file".into());
    }
    let unreadable = |e: std::io::Error| format!("unreadable ({e})");
    let mut bytes = Vec::new();
    File::open(at).map_err(unreadable)?.take(MAX_BYTES + 1).read_to_end(&mut bytes).map_err(unreadable)?;
    Ok(bytes)
}
