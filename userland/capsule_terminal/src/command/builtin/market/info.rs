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

//! `market info <id>`: one listing in full, and every install gate with
//! its own verdict, so a refusal names what refused.

use alloc::format;
use alloc::string::String;

use nonos_libc::mk_app_install_status;
use nonos_market_proto::{
    listing_body, pair_body, parse_app, parse_release, Stage, OP_GET_APP, OP_GET_RELEASE,
};

use super::call::call;
use super::failure::Failure;
use super::ready::{gates, readiness};
use super::stage_text::stage_text;
use super::wrap::wrap;
use crate::command::output::Output;

/// Room for a description line under the two-space indent.
const WIDTH: usize = 88;

pub(super) fn run(out: &mut Output<'_>, id: &[u8]) {
    let app = match listing_body(id).ok_or(Failure::Malformed).and_then(|b| call(OP_GET_APP, &b)) {
        Ok(body) => match parse_app(&body) {
            Some(app) => app,
            None => return Failure::Malformed.say(out, ""),
        },
        Err(why) => return why.say(out, super::failure::NO_LISTING),
    };
    let text = |b: &[u8]| String::from_utf8_lossy(b).into_owned();
    out.writeln(format!("{}  ({})", text(&app.name), text(id)).as_bytes());
    out.writeln(format!("  publisher  {}", text(&app.publisher)).as_bytes());
    let release = pair_body(id, b"").and_then(|b| call(OP_GET_RELEASE, &b).ok());
    match release.and_then(|b| parse_release(&b)) {
        Some(r) => {
            out.writeln(format!("  version    {}", text(r.short_version())).as_bytes());
            if !r.note.is_empty() {
                out.writeln(format!("  checked    {}", text(&r.note)).as_bytes());
            }
        }
        None => out.writeln(b"  version    the market named no release"),
    }
    for line in wrap(&app.description, WIDTH) {
        out.writeln(format!("  {}", text(line)).as_bytes());
    }
    match readiness(id).ok().flatten() {
        Some(r) => gates(out, &r),
        None => out.writeln(b"  gates      the market did not say"),
    }
    if id.starts_with(b"linux.") {
        let stage = Stage::of(mk_app_install_status(id));
        out.writeln(format!("  status     {}", stage_text(id, stage)).as_bytes());
    }
}
