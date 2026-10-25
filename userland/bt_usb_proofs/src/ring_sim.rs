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

//! A keyring answering as `capsule_keyring`'s handlers do (`store.rs`,
//! `retrieve.rs`, `delete.rs`, `server/caller.rs`): their layouts, errnos
//! and pid check; with OP_LIST as lead unit L33 gives it, or without.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use crate::ring::keyring::{Call, Keyring};

const EINVAL: i32 = -22;
const EACCES: i32 = -13;
pub const NO_ANSWER: i32 = -11;

#[derive(Default)]
pub struct Ring {
    /// id to (owner, key type, data).
    pub records: BTreeMap<u32, (u32, u8, Vec<u8>)>,
    pub next: u32,
    /// Whether L33's OP_LIST is there.
    pub lists: bool,
    /// Every request: op and payload.
    pub requests: Vec<(u16, Vec<u8>)>,
}

/// One capsule's line to the ring, as the kernel stamps its pid.
pub struct Line(pub Rc<RefCell<Ring>>, pub u32);

pub fn ring(lists: bool) -> Rc<RefCell<Ring>> {
    Rc::new(RefCell::new(Ring { next: 1, lists, ..Ring::default() }))
}

/// A capsule with pid `pid` that names itself truly.
pub fn capsule(ring: &Rc<RefCell<Ring>>, pid: u32) -> Keyring<Line> {
    Keyring::new(Line(ring.clone(), pid), pid)
}

impl Call for Line {
    fn call(&mut self, op: u16, p: &[u8], room: usize) -> Result<Vec<u8>, i32> {
        let mut r = self.0.borrow_mut();
        r.requests.push((op, p.to_vec()));
        let sender = self.1;
        let named = p.get(..4).map(|b| u32::from_le_bytes(b.try_into().unwrap()));
        let answer = match (op, named) {
            (_, None) => Err(EINVAL),
            (_, Some(pid)) if pid != sender => Err(EACCES),
            (1, _) => r.store(sender, p),
            (2 | 3, _) if p.len() != 8 => Err(EINVAL),
            (2 | 3, _) => r.by_id(op, sender, u32::from_le_bytes(p[4..8].try_into().unwrap())),
            (31, _) if r.lists && p.len() == 5 => Ok(r.list(sender, p[4])),
            _ => Err(EINVAL),
        };
        answer.and_then(|a| if a.len() <= room { Ok(a) } else { Err(NO_ANSWER) })
    }

    fn now_ms(&mut self) -> u64 {
        4242
    }
}
