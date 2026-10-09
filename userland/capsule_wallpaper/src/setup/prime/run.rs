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
use crate::paint::{decode_jpeg, fill_argb, paint_image};
use crate::policy_client::lookup_policy;
use crate::server::scene::first_submit;
use crate::state::{Backoff, Context, FadeTimeline, Policy};
use crate::subscriber::job::Plan;

// Ink, the desktop's darkest tone, shows if no image decodes.
const DEFAULT_ARGB: u32 = 0xFF0A_0B0D;
const EMBEDDED_WALLPAPER: &[u8] =
    include_bytes!("../../../../../nonos-data/wallpapers/special-variant-9.jpg");

pub fn run() -> Result<Context, &'static str> {
    let compositor_port = discover::lookup_compositor_port()?;
    healthcheck(compositor_port, 1)?;
    let backing = backing::allocate(compositor_port, 2)?;
    fill_argb(backing.backing_va, backing.stride, backing.width, backing.height, DEFAULT_ARGB);
    let surface_handle = register::share_surface(&backing)?;
    let mut ctx = Context {
        compositor_port,
        width: backing.width,
        height: backing.height,
        stride: backing.stride,
        backing_va: backing.backing_va,
        surface_handle,
        registered: false,
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
    if let Some(img) = decode_jpeg(EMBEDDED_WALLPAPER) {
        let _ = paint_image(&ctx, &img);
    }
    // Unanswered, the service loop asks again while it serves.
    first_submit(&mut ctx);
    // The chosen wallpaper is not fetched here. Setup used to fetch and
    // decode it before the service answered anyone, up to 15 s for the first
    // chunk alone, and the desktop shell's setup gave up on its calls ("setup
    // stuck: wallpaper call failed"). The built in picture is up now; the
    // first turn of the service loop asks the policy and starts a worker on
    // the chosen one (subscriber/tick.rs).
    Ok(ctx)
}
