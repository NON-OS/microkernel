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

//! A `git clone` job set up: the url and branch read, the network said, nothing sent yet.

use alloc::format;
use alloc::string::String;

use nonos_http::parse_url;
use nonos_route_link::Route;
use nonos_tls::rtc_now;

use super::super::super::repo::storage;
use super::{CloneJob, Step};
use crate::command::output::Output;
use crate::git::Stepped;
use crate::term::state::State;

pub fn prepare(state: &mut State, argv: &[&[u8]]) -> Option<CloneJob> {
    let Some(url) = argv.first().and_then(|a| core::str::from_utf8(a).ok()) else {
        Output::new(&mut state.scrollback).writeln(b"usage: git clone <https url> [branch]");
        return None;
    };
    let Some(remote) = parse_url(url) else {
        Output::new(&mut state.scrollback).writeln(b"git clone: only https urls are supported");
        return None;
    };
    let branch = argv.get(1).and_then(|a| core::str::from_utf8(a).ok()).unwrap_or("main");
    let Some(into) = remote.last_segment().map(String::from) else {
        Output::new(&mut state.scrollback).writeln(b"git clone: that url names no directory");
        return None;
    };
    let route = Route::chosen();
    // The chosen network is not running: say so before anything is written.
    if let Route::Down(why) = route {
        let line = format!("git clone: {why}, so nothing was sent");
        Output::new(&mut state.scrollback).writeln(line.as_bytes());
        return None;
    }
    let line = format!("Cloning into {into} {}", route.name());
    Output::new(&mut state.scrollback).writeln(line.as_bytes());
    Some(CloneJob {
        transport: Stepped::new(remote),
        storage: storage(state),
        route,
        rtc: rtc_now(),
        url: String::from(url),
        into,
        branch: String::from(branch),
        step: Some(Step::Refs),
        carrying: None,
        reported: 0,
    })
}
