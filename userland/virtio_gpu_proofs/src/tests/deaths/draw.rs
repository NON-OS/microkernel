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
//! The owner's picture: the primary filled with SECRET and shown on scanout
//! 0, and a resource of its own (backed inside the primary, as the driver
//! allows) holding SECRET too, shown on scanout 1.

use super::ask::{ask, with64, words, OWNER};
use super::rig::Rig;
use crate::protocol::{
    OP_ATTACH_BACKING, OP_CREATE_RESOURCE, OP_FLUSH, OP_GET_PRIMARY_SURFACE, OP_SET_SCANOUT,
    OP_TRANSFER_TO_HOST,
};

pub const SECRET: u32 = 0x5EC2_E7ED;
pub const SIDE: u32 = 64;

/// Shows `pid`'s whole primary as it is now.
pub fn present(r: &Rig, pid: u32, now: u64) {
    let p = r.d.primary.unwrap();
    let (id, w, h) = (p.resource_id, p.width, p.height);
    let sent = ask(r, pid, OP_TRANSFER_TO_HOST, &with64(words(&[id, 0, 0, w, h]), 24, 0), now);
    assert_eq!(sent.0, 0);
    assert_eq!(ask(r, pid, OP_SET_SCANOUT, &words(&[0, id, 0, 0, w, h]), now).0, 0);
    assert_eq!(ask(r, pid, OP_FLUSH, &words(&[id, 0, 0, w, h]), now).0, 0);
}

pub fn fill(r: &Rig, v: u32) {
    let p = r.d.primary.unwrap();
    let n = (p.stride * p.height / 4) as usize;
    // SAFETY: the primary's mapping, held by the shim for the rig's life.
    unsafe { std::slice::from_raw_parts_mut(p.user_va as *mut u32, n) }.fill(v);
}

pub fn primary_px(r: &Rig) -> Vec<u32> {
    let p = r.d.primary.unwrap();
    let n = (p.backing_len / 4) as usize;
    // SAFETY: as for `fill`.
    unsafe { std::slice::from_raw_parts(p.user_va as *const u32, n) }.to_vec()
}

/// The owner asks for the primary, makes its own resource and shows both.
/// Its resource's id.
pub fn draw(r: &Rig) -> u32 {
    let p = r.d.primary.unwrap();
    assert_eq!(ask(r, OWNER, OP_GET_PRIMARY_SURFACE, &[], 0).0, 0);
    let made = ask(r, OWNER, OP_CREATE_RESOURCE, &words(&[0, 1, SIDE, SIDE]), 0);
    assert_eq!(made.0, 0);
    let own = u32::from_le_bytes(made.1[..4].try_into().unwrap());
    let back = with64(with64(words(&[own, 0]), 8, p.backing_addr), 16, (SIDE * SIDE * 4) as u64);
    assert_eq!(ask(r, OWNER, OP_ATTACH_BACKING, &back, 0).0, 0);
    fill(r, SECRET);
    present(r, OWNER, 0);
    let rect = words(&[own, 0, 0, SIDE, SIDE]);
    assert_eq!(ask(r, OWNER, OP_TRANSFER_TO_HOST, &with64(rect.clone(), 24, 0), 0).0, 0);
    assert_eq!(ask(r, OWNER, OP_SET_SCANOUT, &words(&[1, own, 0, 0, SIDE, SIDE]), 0).0, 0);
    assert_eq!(ask(r, OWNER, OP_FLUSH, &rect, 0).0, 0);
    let g = r.gpu.borrow();
    assert!(g.shown(0).unwrap().iter().all(|&p| p == SECRET), "the secret on scanout 0");
    assert!(g.shown(1).unwrap().iter().all(|&p| p == SECRET), "and on scanout 1");
    own
}
