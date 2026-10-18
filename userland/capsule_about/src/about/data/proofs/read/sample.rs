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

//! One sample of everything the board shows, the table read before the registry.

use super::super::{census::*, firmware::*};
use super::{board::board, boot::*, entries::*, snapshot::*};
use crate::about::data::verify::{each, name_of};
use alloc::vec::Vec;
use nonos_libc::PROC_NAME_LEN;
use nonos_route_proof::route_verdict;

pub fn read() -> Snapshot {
    let boot = boot();
    let mut rows: Vec<Row> = Vec::new();
    let table = each(|e| {
        let name = name_of(e);
        let mut n = [0u8; PROC_NAME_LEN];
        n[..name.len()].copy_from_slice(name);
        rows.push(Row {
            pid: e.pid,
            state: e.state,
            uptime_ms: e.uptime_ms,
            caps: e.caps,
            name: n,
            len: name.len(),
        });
    });
    // The registry is read after the table, so a capsule in the table was
    // spawned, and recorded, before the registry read began.
    let registry = entries();
    let (census, holders) = match (table, registry) {
        (true, Some(reg)) => {
            let procs: Vec<Proc<'_>> = rows
                .iter()
                .map(|r| Proc {
                    pid: r.pid,
                    state: r.state,
                    uptime_ms: r.uptime_ms,
                    caps: r.caps,
                    name: &r.name[..r.len],
                })
                .collect();
            let admitted: Vec<Admitted> = reg.iter().map(|e| e.0).collect();
            let holders = reg
                .iter()
                .filter(|(a, _)| a.caps & CAP_NETWORK != 0 && !is_firmware(a.pid))
                .map(|(a, m)| holder(a, m, &rows))
                .collect();
            (Some(census(&procs, &admitted)), holders)
        }
        _ => (None, Vec::new()),
    };
    let chosen = chosen_route();
    let board = board();
    let latest = match (chosen, board) {
        (Some(nonos_route_proof::ROUTE_NYM), Some((nym, _))) => nym,
        (Some(nonos_route_proof::ROUTE_ANYONE), Some((_, anyone))) => anyone,
        _ => None,
    };
    Snapshot { boot, census, holders, chosen, board, route: route_verdict(chosen, latest) }
}
