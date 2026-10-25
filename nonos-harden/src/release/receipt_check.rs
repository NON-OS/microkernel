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

//! `release.receipt`: the receipt the seal wrote agrees with the release. The
//! seal found the roots it measured embedded, measured no artifact that was
//! missing, and every artifact it measured is among the bytes the record
//! sealed. The receipt also names the commit `tree.source` holds the tree to.

use super::read::read_regular;
use super::receipt::{parse, FILE, MAX_BYTES};
use super::receipt_agree::{disagreements, limits};
use crate::check::Check;
use crate::ctx::Ctx;

pub const ID: &str = "release.receipt";

pub fn check(ctx: &mut Ctx) -> Check {
    let Some(rec) = &ctx.record else {
        return Check::not_run(ID, "no release record to hold the receipt against");
    };
    let r = match read_regular(&ctx.release.join(FILE), MAX_BYTES).and_then(|b| parse(&b)) {
        Ok(r) => r,
        Err(e) => return Check::fail(ID, format!("{FILE}: {e}")),
    };
    let (bad, limits) = (disagreements(&r, rec), limits(&r));
    let line =
        format!("built from commit {}; every artifact the seal measured is sealed", r.commit);
    ctx.receipt = Some(r);
    match bad.is_empty() {
        true => Check::pass(ID, line).with_limits(limits),
        false => Check::fail(ID, bad.join("; ")),
    }
}
