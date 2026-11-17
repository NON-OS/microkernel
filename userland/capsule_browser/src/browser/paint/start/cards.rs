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

use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::widgets::Ink;

use super::card::{paint as card, Line};
use crate::browser::net::mixnet::{self, Network};
use crate::browser::paint::toolbar::route_of;
use crate::browser::shell::dl_line::{count_line, status_line};
use crate::browser::shell::route::Route;
use crate::browser::shell::visits::top_sites;
use crate::browser::shell::Shell;
use crate::browser::state::State;
use crate::browser::ui::start_geom::StartGeom;

const TOP_SITES: usize = 5;
const EMPTY: [Line<'static>; 2] =
    [("Sites you visit this session", Ink::Muted, None), ("appear here.", Ink::Muted, None)];

/// Frequently visited, Network and This session, side by side or stacked.
/// This session counts tabs, visits and downloads, the newest download's
/// line under them.
pub fn paint(state: &State, shell: &Shell, g: &StartGeom, fb: &mut PaintBuffer) {
    let [freq, net, session] = g.cards;
    let top = top_sites(&shell.visits, TOP_SITES);
    let counts: Vec<String> = top.iter().map(|(_, n)| format!("{}", n)).collect();
    let mut lines: Vec<Line> = Vec::new();
    for ((host, _), n) in top.iter().zip(&counts) {
        lines.push((host, Ink::Text2, Some(n)));
    }
    card(fb, freq, "Frequently visited", if top.is_empty() { &EMPTY } else { &lines });
    let route = route_of(state);
    let ink = if matches!(route, Route::Mixnet | Route::Anyone) { Ink::Ok } else { Ink::Muted };
    let end = endpoint(state);
    card(fb, net, "Network", &[(route.name(), ink, None), (&end, Ink::Muted, None)]);
    let tabs = format!("Open tabs: {}", shell.cards.len());
    let visits = format!("Pages visited: {}", shell.visits.len());
    let (dl, last) = (count_line(&shell.downloads), shell.downloads.last().map(status_line));
    let mut lines: Vec<Line> = vec![(&tabs, Ink::Text2, None), (&visits, Ink::Text2, None)];
    lines.push((&dl, Ink::Text2, None));
    lines.extend(last.as_deref().map(|s| (s, Ink::Muted, None)));
    lines.push(("History clears at shutdown", Ink::Muted, None));
    card(fb, session, "This session", &lines);
}

/* Where the network the next request takes leads. */
fn endpoint(state: &State) -> String {
    match (mixnet::chosen(), state.proxy.as_ref()) {
        (Network::Nym, _) => String::from("Through the Nym mixnet"),
        (Network::Anyone, _) => String::from("Through the Anyone network"),
        (Network::Direct, Some(p)) => format!("SOCKS5 proxy {}:{}", p.host, p.port),
        (Network::Direct, None) => String::from("Not anonymised"),
    }
}
