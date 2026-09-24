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

//! The catalogue, as the market capsule serves it.

use alloc::vec::Vec;

use crate::store::listing::Listing;

use super::wire::call;

const OP_LIST_APPS: u16 = 2;

pub fn fetch(port: u32, request_id: u32) -> Option<Vec<Listing>> {
    let body = call(port, OP_LIST_APPS, request_id, &[])?;
    let count = u32::from_le_bytes(body.get(..4)?.try_into().ok()?) as usize;
    let mut at = 4;
    let mut out = Vec::with_capacity(count.min(1024));
    for _ in 0..count {
        let (id, next) = lp(&body, at)?;
        at = next;
        let measurement: [u8; 32] = body.get(at..at + 32)?.try_into().ok()?;
        at += 32;
        let (name, next) = lp(&body, at)?;
        at = next;
        let ready = *body.get(at)? != 0;
        at += 1;
        out.push(Listing::new(id, measurement, name, ready));
    }
    Some(out)
}

/// A length-prefixed string: four bytes of length, then the bytes.
fn lp(body: &[u8], at: usize) -> Option<(Vec<u8>, usize)> {
    let len = u32::from_le_bytes(body.get(at..at + 4)?.try_into().ok()?) as usize;
    let start = at + 4;
    let end = start.checked_add(len)?;
    Some((body.get(start..end)?.to_vec(), end))
}
