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
use crate::clients::compositor;
use crate::discover::Peers;

use super::boot::BootedApp;
use super::repaint::repaint;
use super::request_id::next;

const APP_LAYER_Z: u32 = 2;

/// Put a minimized window back in the scene where it was and repaint it.
pub(super) fn restore<A: App>(booted: &mut BootedApp<A>, peers: &Peers, request_id: &mut u32) {
    let _ = compositor::scene_submit(
        peers.compositor,
        next(request_id),
        booted.binding.surface_handle,
        booted.binding.x,
        booted.binding.y,
        booted.binding.width,
        booted.binding.height,
        APP_LAYER_Z,
    );
    booted.minimized = false;
    repaint(booted, peers, request_id);
}
