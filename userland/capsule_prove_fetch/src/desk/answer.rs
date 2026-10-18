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

//! One request against the desk. PUT answers app.store only; GET and STATUS
//! app.store and market.index. Anyone else is EPERM, before its
//! body is read.

use alloc::vec::Vec;

use nonos_proof_abi::wire::fetch::{anchor_of_body, get_of, Put, GET, PUT, STATUS};
use nonos_proof_abi::wire::Header;

use super::errno::{EINVAL, ENOENT, EPERM};
use super::transfer::Desk;

/// The caller as the kernel's service registry names it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Caller {
    pub pid: u32,
    /// The pid owns app.store.
    pub store: bool,
    /// The pid owns market.index.
    pub index: bool,
}

pub fn answer(desk: &mut Desk, c: Caller, h: &Header, body: &[u8]) -> (i32, Vec<u8>) {
    let none = |s: i32| (s, Vec::new());
    let allowed = match h.op {
        PUT => c.store,
        GET | STATUS => c.store || c.index,
        _ => return none(EINVAL),
    };
    if !allowed {
        return none(EPERM);
    }
    match h.op {
        PUT => match Put::decode(body).map(|p| desk.put(c.pid, &p)) {
            Some(Ok(a)) => (0, a.map_or(Vec::new(), |a| a.to_vec())),
            Some(Err(s)) => none(s),
            None => none(EINVAL),
        },
        GET => match get_of(body).map(|(l, a, at)| desk.get(&l, &a, at)) {
            Some(Ok(b)) => (0, b),
            Some(Err(s)) => none(s),
            None => none(EINVAL),
        },
        _ => match anchor_of_body(body).map(|a| desk.status(&a)) {
            Some((s, b)) if s != ENOENT => (s, b.to_vec()),
            Some((s, _)) => none(s),
            None => none(EINVAL),
        },
    }
}
