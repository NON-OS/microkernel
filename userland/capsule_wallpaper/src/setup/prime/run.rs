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

use super::super::discover;
use super::{backing, register};
use crate::catalog_client::lookup_catalog;
use crate::compositor_client::healthcheck;
use crate::first_paint::job::DEFAULT_ARGB;
use crate::policy_client::lookup_policy;
use crate::state::{Backoff, Context, FadeTimeline, Policy};
use crate::subscriber::job::Plan;

pub fn run() -> Result<Context, &'static str> {
    let compositor_port = discover::lookup_compositor_port()?;
    healthcheck(compositor_port, 1)?;
    let backing = backing::allocate(compositor_port, 2)?;
    let surface_handle = register::share_surface(&backing, discover::lookup_compositor_pid()?)?;
    let mut ctx = Context {
        compositor_port,
        width: backing.width,
        height: backing.height,
        stride: backing.stride,
        backing_va: backing.backing_va,
        surface_handle,
        registered: false,
        first_paint_out: false,
        register_backoff: Backoff::new(),
        commit_pending: false,
        commit_backoff: Backoff::new(),
        argb: DEFAULT_ARGB,
        alpha: 0xFF,
        policy: Policy::Fill,
        fade: FadeTimeline::new(),
        next_request_id: 3,
        policy_port: lookup_policy(),
        catalog_port: lookup_catalog(),
        plan: Plan::new(),
        subscriber_ticks: 0,
    };
    ctx.set_argb(DEFAULT_ARGB);
    // Painted by a worker while the service answers; the service loop puts
    // it in the compositor's scene once it is up (first_paint/).
    crate::first_paint::start(&mut ctx);
    // The chosen wallpaper is not fetched here. Setup used to fetch and
    // decode it before the service answered anyone, up to 15 s for the first
    // chunk alone, and the desktop shell's setup gave up on its calls ("setup
    // stuck: wallpaper call failed"). The built in picture is up now; the
    // first turn of the service loop asks the policy and starts a worker on
    // the chosen one (subscriber/tick.rs).
    Ok(ctx)
}
