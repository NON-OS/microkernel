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

//! `release.kind`: what the record says this image is. A development image
//! (a profile that is not a release) can still be GO, and the verdict then
//! says it is not a release. A profile that claims to be a release while it
//! carries the development attestation contradicts itself, and fails.

use crate::check::Check;
use crate::ctx::{Ctx, Kind};

pub const ID: &str = "release.kind";
/// The kernel feature a development image is sealed with: path-only
/// attestation, no STARK proof (tools/nonos-dev-image).
const DEV_ATTEST: &str = "nonos-dev-attest";

pub fn check(ctx: &mut Ctx) -> Check {
    let Some(rec) = &ctx.record else {
        return Check::not_run(ID, "no release record to read it from");
    };
    let dev_attest = rec.features.iter().any(|f| f == DEV_ATTEST);
    if rec.release && dev_attest {
        let why = format!("profile {} says it is a release but carries {DEV_ATTEST}", rec.profile);
        return Check::fail(ID, why);
    }
    ctx.kind = if rec.release { Kind::Release } else { Kind::Development };
    let what = if rec.release { "a release" } else { "a development image, not a release" };
    Check::pass(ID, format!("profile {}: {what}", rec.profile))
}
