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

//! The kept displays put back on the compositor (state/display_keep.rs), on
//! a tick the policy store answered: each field read, and sent on as the
//! same compositor op the Settings relay uses, when the rule says it is due.
//! The fields are named by id: the store refuses them until it keeps them,
//! and then nothing is sent.

use nonos_policy_client::call_within;
use nonos_policy_proto::{IPC_PAYLOAD_MAX, KIND_U8, OP_GET};

use super::handlers::launcher_request::lookup_pid;
use super::handlers::relay_table::relayed;
use crate::compositor_client::wire::call_boot;
use crate::protocol::{OP_DISPLAY_LAYOUT, OP_DISPLAY_SCALE};
use crate::state::display_keep::{body, FIELDS};
use crate::state::Context;

const COMPOSITOR: &[u8] = b"compositor";
/// The tick's other reads wait no longer than this.
const READ_MS: u64 = 64;

fn read(port: u32, field: u32) -> Option<u8> {
    let mut rx = [0u8; IPC_PAYLOAD_MAX];
    let r = call_within(port, OP_GET, field, KIND_U8, &mut rx, READ_MS)?;
    let ours = r.header.kind == KIND_U8 && r.header.field == field;
    r.payload.first().copied().filter(|_| ours)
}

pub fn replay(ctx: &mut Context) {
    let Some(pid) = lookup_pid(COMPOSITOR) else { return };
    ctx.readings.displays.compositor(pid);
    for (i, &field) in FIELDS.iter().enumerate() {
        let read = read(ctx.ports.policy_port, field);
        let Some(v) = ctx.readings.displays.due(i, read) else { continue };
        let shell_op = if i == 0 { OP_DISPLAY_LAYOUT } else { OP_DISPLAY_SCALE };
        let (Some((b, len)), Some((op, _))) = (body(i, v), relayed(shell_op)) else { continue };
        let rid = ctx.issue_request_id();
        // Any answer, taken or refused: only a missing one is asked again.
        if call_boot(ctx.ports.compositor_port, op, rid, &b[..len]).is_ok() {
            ctx.readings.displays.told(i, v);
        }
    }
}
