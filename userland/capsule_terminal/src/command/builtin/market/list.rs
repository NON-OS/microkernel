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

//! `market list`: one line a listing, with where it stands on this machine.

use nonos_libc::mk_app_install_status;
use nonos_market_proto::{parse_list, Entry, Stage, OP_LIST_APPS};

use super::call::call;
use super::failure::Failure;
use super::list_row::{heading, row};
use crate::command::output::Output;

pub(super) fn run(out: &mut Output<'_>) {
    let body = match call(OP_LIST_APPS, &[]) {
        Ok(body) => body,
        Err(why) => return why.say(out, "this machine has no signed catalogue"),
    };
    let Some(entries) = parse_list(&body) else {
        return Failure::Malformed.say(out, "");
    };
    let (plain, styled) = heading(entries.len());
    out.writeln_styled(&plain, &styled);
    let width = entries.iter().map(|e| e.id.len()).max().unwrap_or(0);
    for e in &entries {
        let (plain, styled) = row(&e.id, &e.name, standing(e), width);
        out.writeln_styled(&plain, &styled);
    }
    if !entries.is_empty() {
        let hint = b"`market info <id>` for one in full, `market install <id>` to install it";
        out.writeln_styled(hint, &[b"\x1b[2m", &hint[..], b"\x1b[0m"].concat());
    }
}

/// What the machine says first, then the market's verdict. Only a Linux
/// listing is installed by the system, so only one is asked about.
fn standing(e: &Entry) -> &'static str {
    let stage = match e.id.starts_with(b"linux.") {
        true => Stage::of(mk_app_install_status(&e.id)),
        false => Stage::Idle,
    };
    match stage {
        Stage::Installed => "installed",
        Stage::Removing => "removing",
        s if s.pending() => "installing",
        Stage::Refused | Stage::Failed(_) => "failed",
        _ if e.ready => "ready",
        _ => "not ready",
    }
}
