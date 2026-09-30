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

use nonos_libc::mk_uptime_ms;

use crate::app::{App, AppManifest};
use crate::discover::Peers;
use crate::setup::{ensure_input_subscription, open_window, WindowBinding};

use super::drag::DragState;
use super::held::Held;
use super::prime_frame::prime_frame;

pub(super) const INITIAL_PAINT_ATTEMPTS: usize = 256;

pub(super) struct BootedApp<A: App> {
    pub app: A,
    pub manifest: AppManifest,
    pub binding: WindowBinding,
    pub input_ready: bool,
    /// Uptime of the last input subscription attempt, for the heartbeat.
    pub input_beat_ms: i64,
    /// A message a paced wait received that no drain has handled yet.
    pub held: Option<Held>,
    pub primed: bool,
    pub maximized: bool,
    pub minimized: bool,
    pub saved: (u32, u32, u32, u32),
    pub drag: DragState,
}

pub(super) fn boot<A: App>(
    mut app: A,
    peers: &Peers,
    request_id: &mut u32,
) -> Result<BootedApp<A>, &'static str> {
    let manifest = super::fit_display::fit_to_display(app.manifest(), peers, request_id);
    let binding = open_window(peers, &manifest, request_id)?;
    let input_ready = ensure_input_subscription(peers.input_router, &manifest, request_id);
    let primed = prime_frame(&mut app, &manifest, &binding, peers, request_id);
    Ok(BootedApp {
        app,
        manifest,
        binding,
        input_ready,
        input_beat_ms: mk_uptime_ms(),
        held: None,
        primed,
        maximized: false,
        minimized: false,
        saved: (0, 0, 0, 0),
        drag: DragState::new(),
    })
}
