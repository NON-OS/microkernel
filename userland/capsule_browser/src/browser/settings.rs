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

//! The settings panel behind the menu button. It says which network requests
//! leave through, lets the reader choose Direct, Nym or Anyone for the next
//! request, and keeps the manual SOCKS5 proxy for the direct network (which
//! focuses the address bar with the command prefix, reusing the address input).

use alloc::format;
use alloc::string::String;

use nonos_app_skeleton::{EventOutcome, PaintBuffer};

use crate::browser::manifest::WIDTH;
use crate::browser::net::mixnet::{self, Network};
use crate::browser::paint::chrome::constants;
use crate::browser::state::State;

const PANEL_W: i32 = 300;
const PANEL_H: i32 = 230;
const PANEL_TOP: i32 = constants::TITLEBAR as i32 + 56;
const ROW_H: i32 = 30;
const ROW_STEP: i32 = 36;
const FIRST_ROW_Y: i32 = PANEL_TOP + 70;
// The three networks, top to bottom, then the proxy row.
const ROWS: [Network; 3] = [Network::Direct, Network::Nym, Network::Anyone];
const PROXY_Y: i32 = FIRST_ROW_Y + 3 * ROW_STEP;

fn panel_x(width: i32) -> i32 {
    width - PANEL_W - 12
}

fn width_of(state: &State) -> i32 {
    if state.viewport_w > 0 {
        state.viewport_w as i32
    } else {
        WIDTH as i32
    }
}

/// What the panel and the address bar say the next request leaves through.
/// A manual proxy only carries traffic on the direct network, so it is named
/// there and nowhere else.
pub fn network_line(state: &State) -> String {
    let net = mixnet::chosen();
    match (net, state.proxy.as_ref()) {
        (Network::Direct, Some(p)) => {
            format!("Network: direct, SOCKS5 proxy {}:{}", p.host, p.port)
        }
        _ => format!("Network: {}", net.label()),
    }
}

/// Draw the panel over the page when it is open.
pub fn paint(state: &State, fb: &mut PaintBuffer) {
    if !state.settings_open {
        return;
    }
    let x = panel_x(fb.width as i32);
    fb.fill_rect(x as u32, PANEL_TOP as u32, PANEL_W as u32, PANEL_H as u32, constants::TOOLBAR_BG);
    fb.fill_rect(x as u32, PANEL_TOP as u32, PANEL_W as u32, 2, constants::ACCENT);
    fb.text_ttf(x + 14, PANEL_TOP + 12, "Settings", constants::FG, 16.0);
    fb.text_ttf(x + 14, PANEL_TOP + 42, &network_line(state), constants::DIM, 14.0);

    let chosen = mixnet::chosen();
    for (i, net) in ROWS.iter().enumerate() {
        let y = FIRST_ROW_Y + i as i32 * ROW_STEP;
        button(fb, x, y, net.label(), *net == chosen);
    }
    let proxy_label =
        if state.proxy.is_some() { "Turn proxy off" } else { "Set proxy (type host:port)" };
    button(fb, x, PROXY_Y, proxy_label, false);
}

fn button(fb: &mut PaintBuffer, x: i32, y: i32, label: &str, active: bool) {
    fb.fill_rect(
        (x + 12) as u32,
        y as u32,
        (PANEL_W - 24) as u32,
        ROW_H as u32,
        constants::FIELD_BG,
    );
    if active {
        fb.fill_rect((x + 12) as u32, y as u32, 4, ROW_H as u32, constants::ACCENT);
    }
    let color = if active { constants::ACCENT } else { constants::FG };
    fb.text_ttf(x + 24, y + 8, label, color, 14.0);
}

enum Action {
    Choose(Network),
    Proxy,
    Close,
    Ignore,
}

fn action_at(x: i32, y: i32, width: i32) -> Action {
    let px = panel_x(width);
    let inside = x >= px && x < px + PANEL_W && y >= PANEL_TOP && y < PANEL_TOP + PANEL_H;
    if !inside {
        return Action::Close;
    }
    for (i, net) in ROWS.iter().enumerate() {
        let ry = FIRST_ROW_Y + i as i32 * ROW_STEP;
        if y >= ry && y < ry + ROW_H {
            return Action::Choose(*net);
        }
    }
    if y >= PROXY_Y && y < PROXY_Y + ROW_H {
        return Action::Proxy;
    }
    Action::Ignore
}

/// Handle a click while the panel is open. A click on empty space, or anywhere
/// outside the panel, closes it.
///
/// Choosing a network changes where the next request goes and nothing else:
/// the route itself is taken when that request starts, so a page already
/// loading finishes on the network it started on.
pub fn on_click(state: &mut State, x: i32, y: i32) -> EventOutcome {
    match action_at(x, y, width_of(state)) {
        Action::Choose(net) => {
            mixnet::choose(net);
            state.settings_open = false;
            state.status = format!("{} from the next request", net.label());
        }
        Action::Proxy if state.proxy.is_some() => {
            state.proxy = None;
            state.settings_open = false;
            state.status = String::from("proxy off");
        }
        Action::Proxy => {
            state.settings_open = false;
            state.address = String::from("proxy socks5://");
            state.address_focused = true;
            state.status = String::from("type host:port then press Enter");
        }
        Action::Close => {
            state.settings_open = false;
        }
        Action::Ignore => {}
    }
    EventOutcome::Repaint
}
