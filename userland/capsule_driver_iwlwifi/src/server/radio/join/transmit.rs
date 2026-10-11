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
//! The two ends of a connect: its transmit region, and what it records.

use crate::driver::Driver;
use crate::firmware::gen3::join::exchange::Progress;
use crate::firmware::gen3::join::run::{JoinEnd, Joined};
use crate::firmware::gen3::txq::TX_REGION;
use crate::server::join_wire::{end_code, ConnectResult};

use super::super::grant::Grant;
use super::super::say::Line;
use super::state::Join;

/// The transmit region: one broker grant on the first join, kept from then on.
pub(super) fn tx_region(join: &mut Join, d: &Driver) -> Option<&'static Grant> {
    let tx = match join.tx {
        Some(t) => t,
        None => match Grant::map(d.device_id, d.claim_epoch, TX_REGION) {
            Some(g) => {
                let t: &'static Grant = alloc::boxed::Box::leak(alloc::boxed::Box::new(g));
                join.tx = Some(t);
                t
            }
            None => {
                Line::new().text(b"join: no DMA region for the transmit queues").send();
                return None;
            }
        },
    };
    Some(tx)
}
/// What a connect answers once the join ran; an opened port is kept, with
/// the network's name for the link reply.
pub(super) fn recorded(
    join: &mut Join,
    ssid: &[u8],
    out: Result<Joined, JoinEnd>,
    progress: Progress,
) -> ConnectResult {
    let mut r = ConnectResult { progress, state: progress.state, ..ConnectResult::default() };
    match out {
        Ok(j) => {
            r.akm = j.akm.suite_type();
            join.ssid[..ssid.len()].copy_from_slice(ssid);
            join.ssid_len = ssid.len();
            join.joined = Some(j);
        }
        Err(e) => (r.code, r.ap_code) = end_code(e),
    }
    r
}
