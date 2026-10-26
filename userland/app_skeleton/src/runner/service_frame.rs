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

use crate::app::App;
use crate::discover::Peers;

use super::apply_drain::apply;
use super::boot::BootedApp;
use super::ensure_primed::ensure_primed;
use super::inbox::{drain, Frame};
use super::refresh_input::refresh_input;

pub(super) fn service_frame<A: App>(
    booted: &mut BootedApp<A>,
    rx: &mut [u8],
    peers: &Peers,
    request_id: &mut u32,
) -> bool {
    refresh_input(booted, peers, request_id);
    if !ensure_primed(booted, peers, request_id) {
        return false;
    }
    // A window whose app asks for full screen takes it from its first frame
    // on, and gives it back when the app stops asking.
    super::asks::follow(booted, peers, request_id);
    let frame = Frame::of(&booted.binding, booted.maximized);
    let (window_id, wm) = (booted.manifest.window_id, peers.wm);
    let ids = (wm, window_id, &mut *request_id);
    let result = drain(&mut booted.app, &mut booted.drag, &mut booted.held, rx, frame, ids);
    apply(booted, peers, request_id, result)
}
