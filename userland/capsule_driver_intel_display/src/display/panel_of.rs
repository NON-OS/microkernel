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

//! The panel a route's monitor describes, and transcoder `t`'s, for
//! callers that want the panel alone.

use super::monitor::Monitor;
use super::monitor_read::read;
use super::panel_fits::fits;
use crate::engine::access::{Clock, Mmio};
use crate::engine::gen::Gen;
use crate::engine::pipe::Transcoder;
use crate::engine::route::Route;
use nonos_gfx_serve::Panel;

pub fn panel(m: &mut dyn Mmio, clock: &dyn Clock, r: &Route) -> Panel {
    let mut blob = [0u8; 256];
    let n = read(m, clock, r, &mut blob);
    Panel::read(&blob[..n], fits)
}

pub fn panel_of(m: &mut dyn Mmio, clock: &dyn Clock, gen: &Gen, t: Transcoder) -> Panel {
    Monitor::of(m, clock, gen, t).panel()
}
