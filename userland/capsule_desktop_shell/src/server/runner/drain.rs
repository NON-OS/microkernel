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

use nonos_libc::{mk_ipc_recv_from, mk_uptime_ms};

use super::constants::{RECV_BLOCK, RECV_RETRY_MS, SERVICE_INBOX};
use super::tick::{wait_ms, wake_due};
use crate::protocol::parse;
use crate::server::respond;
use crate::state::Context;

/// Handle what is in the inbox until it goes quiet or the loop's wake at
/// uptime `wake_ms` (the next tick, or the next toast's expiry) is due.
pub(super) fn drain(ctx: &mut Context, rx: &mut [u8], tx: &mut [u8], wake_ms: i64) {
    loop {
        if wake_due(mk_uptime_ms(), wake_ms) {
            return;
        }
        let mut sender_pid = 0u32;
        let most = if crate::server::ready_to_block::ready_to_block(ctx) {
            RECV_BLOCK
        } else {
            RECV_RETRY_MS
        };
        let timeout = wait_ms(mk_uptime_ms(), wake_ms, most);
        let n =
            mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), timeout, &mut sender_pid);
        crate::server::retry_input_subscription::retry_input_subscription(ctx);
        crate::server::retry_wm_subscription::retry_wm_subscription(ctx);
        crate::server::reap_tray::reap_if_due(ctx);
        crate::server::handlers::installed_launch_poll::poll(ctx);
        if n <= 0 || sender_pid == 0 {
            return;
        }
        if crate::server::wm_notify::handle(ctx, &rx[..n as usize]) {
            continue;
        }
        if crate::server::input::handle(ctx, &rx[..n as usize]) {
            continue;
        }
        let (req, body) = match parse(&rx[..n as usize]) {
            Ok(parsed) => parsed,
            Err((code, req)) => {
                let _ = respond::status(sender_pid, &req, code, tx);
                continue;
            }
        };
        crate::server::dispatch::dispatch(ctx, sender_pid, req, body, tx);
    }
}
