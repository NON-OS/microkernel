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

use crate::compositor_client::push_damage_commit;
use crate::render::layout::menubar_height;
use crate::render::paint_chrome;
use crate::state::indicators::{net, notify_gate, policy};
use crate::state::{Context, NotifyLevel};

pub(super) fn refresh_clock(ctx: &mut Context) {
    let policy_ok = read_policy(ctx);
    crate::sound::service(policy_ok);
    let net_now = net::online();
    if net_now && !ctx.net_was_online && notify_gate::shows(NotifyLevel::Info) {
        ctx.toasts.push(b"network connected", NotifyLevel::Info, crate::server::toast_clock::now());
    }
    ctx.net_was_online = net_now;
    // paint_chrome redraws the whole overlay backing, but only the top bar's
    // pixels actually change each second. Damage just that strip: a full-screen
    // damage every tick would make the compositor recomposite every layer,
    // including any open app window, which is what made the desktop hitch.
    paint_chrome(ctx);
    let rid = ctx.issue_request_id();
    let _ = push_damage_commit(ctx.compositor_port, rid, 0, 0, ctx.width, menubar_height());
}

/*
 * The settings the clock and the notifications follow, read while the policy
 * store answers. The clock format is read first: when that gets no answer the
 * others would wait out their timeouts too, so none is asked until the gap
 * (state/quiet_gap.rs) has passed, and the menu bar keeps the values it had.
 * True when the store answered this tick, so the sound levels may be read.
 */
fn read_policy(ctx: &mut Context) -> bool {
    let now = mk_uptime_ms().max(0) as u64;
    if !ctx.policy_gap.due(now) {
        return false;
    }
    match policy::clock_24h(&mut ctx.policy_port) {
        policy::Read::Absent => false,
        policy::Read::Silent => {
            ctx.policy_gap.missed(now);
            false
        }
        policy::Read::Answered(v) => {
            ctx.policy_gap.answered();
            if let Some(v) = v {
                ctx.clock_24h = v;
            }
            notify_gate::follow(ctx.policy_port);
            if let Some(v) = policy::timezone(ctx.policy_port) {
                ctx.tz_hours = v;
            }
            true
        }
    }
}
