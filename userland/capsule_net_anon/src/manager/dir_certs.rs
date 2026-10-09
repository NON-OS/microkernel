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

//! Fetching and anchoring the authority certificates, one authority at a time.

extern crate alloc;

use alloc::vec::Vec;

use crate::directory::authority::AUTHORITIES;
use crate::directory::fetch::{keys_path, upper};
use crate::directory::verify::{check, parse, AnchorError};
use crate::trace;

use super::cert_rule::wanted;
use super::dir_job::{turn, Turn};
use super::state::Manager;

/// Advance the sweep over the authorities by one turn. `None` while it runs;
/// once every authority has been considered, how many certificates are held.
/// Those are in `state.certs`, by authority index, and are kept from one
/// sweep to the next, so a sweep asks only the authorities still missing.
pub(super) fn sweep(state: &mut Manager, now: u64) -> Option<usize> {
    if state.dir.sweep == 0 && state.dir.job.is_none() {
        /* A certificate past its expiry anchors nothing, so it is dropped here
         * and its authority asked again in this same sweep. */
        state.certs.retain(|(_, cert)| now < cert.expires);
    }
    while state.dir.sweep < AUTHORITIES.len() {
        let index = state.dir.sweep;
        if state.dir.job.is_none() && !wanted(index, &held(state), &state.refetch) {
            state.dir.sweep += 1;
            continue;
        }
        let authority = &AUTHORITIES[index];
        let mut hex = [0u8; 40];
        let width = upper(&mut hex, &authority.v3ident);
        match turn(state, authority.address, authority.dir_port, &keys_path(&hex[..width])) {
            Turn::Busy => return None,
            Turn::Got(body) => anchor(state, index, &body, now),
            Turn::Missed => {}
        }
        state.dir.sweep += 1;
        if state.dir.sweep < AUTHORITIES.len() {
            return None;
        }
    }
    state.dir.sweep = 0;
    state.refetch.clear();
    trace::say_two(b"authority certs", state.certs.len() as u64, AUTHORITIES.len() as u64);
    Some(state.certs.len())
}

fn held(state: &Manager) -> Vec<usize> {
    state.certs.iter().map(|(index, _)| *index).collect()
}

fn anchor(state: &mut Manager, index: usize, body: &[u8], now: u64) {
    let Some(cert) = parse(body) else { return };
    match check(&cert, body, &AUTHORITIES[index].v3ident, now) {
        Ok(()) => {
            /* One certificate per authority: a fresh one replaces any older
             * one still held, so the quorum never counts an authority twice
             * and never checks a signature against a retired key. */
            state.certs.retain(|(held, _)| *held != index);
            state.certs.push((index, cert));
        }
        Err(AnchorError::Expired) => trace::say_num(b"authority signing key expired", index as u64),
        Err(_) => trace::say_num(b"authority cert rejected", index as u64),
    }
}
