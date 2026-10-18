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

//! A request a key made, sent, and the page brought up to date after it.

use nonos_bt_proto::REPLY_MAX;
use nonos_libc::mk_uptime_ms;

use super::call::call;
use super::poll::poll;
use crate::bluetooth::panel::{unreached, Panel, Request};

pub fn run(p: &mut Panel, req: Request) {
    let mut rx = [0u8; REPLY_MAX];
    match call(p, req.op, req.body(), &mut rx) {
        Ok((status, _)) => p.did(&req, status, mk_uptime_ms()),
        Err(reach) => {
            p.lost(reach);
            p.notice = Some(unreached(reach));
            return;
        }
    }
    poll(p, true);
}
