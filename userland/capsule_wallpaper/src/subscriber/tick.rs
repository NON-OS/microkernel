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

use crate::catalog_client::lookup_catalog;
use crate::policy_client::{get_wallpaper, lookup_policy};
use crate::state::Context;

use super::settle::settle;
use super::start::start;

/// Ask the policy which wallpaper it wants this often, in service loop turns.
/// The first turn asks, so the chosen wallpaper's job starts as soon as the
/// service is up.
const POLL_EVERY: u32 = 300;

/// One turn of the service loop: pick up a job's answer, ask the policy when
/// due, and start the next job. Nothing here waits on the catalog.
pub fn tick(ctx: &mut Context) {
    settle(ctx);
    if ctx.subscriber_ticks % POLL_EVERY == 0 {
        poll_policy(ctx);
    }
    ctx.subscriber_ticks = ctx.subscriber_ticks.wrapping_add(1);
    start(ctx);
}

fn poll_policy(ctx: &mut Context) {
    if ctx.policy_port.is_none() {
        ctx.policy_port = lookup_policy();
    }
    if ctx.catalog_port.is_none() {
        ctx.catalog_port = lookup_catalog();
    }
    let Some(policy_port) = ctx.policy_port else {
        return;
    };
    if let Some(wanted) = get_wallpaper(policy_port) {
        ctx.plan.want(wanted);
    }
}
