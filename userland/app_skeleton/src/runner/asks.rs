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

//! What the app asks of its window, followed each frame: full screen
//! (`full_screen_ask.rs`, maximize.rs) and a rect (`rect_ask.rs`,
//! rect_follow.rs), kept together so the runner's state holds one field.

use crate::app::App;
use crate::discover::Peers;

use super::boot::BootedApp;
use super::full_screen_ask::FullScreenAsk;
use super::rect_ask::RectAsk;

pub struct Asks {
    pub full: FullScreenAsk,
    pub rect: RectAsk,
}

impl Asks {
    pub const fn new() -> Self {
        Self { full: FullScreenAsk::new(), rect: RectAsk::new() }
    }
}

/// Follow both asks: full screen first, which a rect then stays out of.
pub(super) fn follow<A: App>(booted: &mut BootedApp<A>, peers: &Peers, request_id: &mut u32) {
    super::window_ops::follow_ask(booted, peers, request_id);
    super::rect_follow::follow_rect(booted, peers, request_id);
}
