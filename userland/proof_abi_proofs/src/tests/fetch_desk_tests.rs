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

//! The fetch desk's callers and one upload: PUT answers app.store only, GET
//! and STATUS market.index too; the chunks of the honest container make its
//! own anchor; meanwhile everything else waits; once held it is served.

use nonos_proof_abi::anchor_of;
use nonos_proof_abi::wire::fetch::{get_body, put_bodies, Stage, GET, PUT, STATUS};
use nonos_proof_abi::wire::{Header, GET_MAX};

use crate::fetch_desk::{answer, Caller, Desk};
use crate::fixture::{honest, LISTING};

pub const STORE: Caller = Caller { pid: 7, store: true, index: false };
pub const INDEX: Caller = Caller { pid: 8, store: false, index: true };
const NOBODY: Caller = Caller { pid: 9, store: false, index: false };

pub fn ask(d: &mut Desk, c: Caller, op: u16, body: &[u8]) -> (i32, Vec<u8>) {
    answer(d, c, &Header { op, flags: 0, id: 1, len: body.len() as u32 }, body)
}

#[test]
fn only_the_store_puts_and_only_the_store_and_index_read() {
    let mut d = Desk::default();
    let put = &put_bodies(&LISTING, &[1; 10])[0];
    assert_eq!(ask(&mut d, INDEX, PUT, put).0, -1);
    assert_eq!(ask(&mut d, NOBODY, PUT, put).0, -1);
    assert_eq!(ask(&mut d, NOBODY, STATUS, &[0; 32]).0, -1);
    assert_eq!(ask(&mut d, NOBODY, GET, &get_body(&LISTING, &[0; 32], 0)).0, -1);
    assert_eq!(ask(&mut d, INDEX, STATUS, &[0; 32]).0, -2, "asked, and not here");
    assert_eq!(ask(&mut d, STORE, 9, &[]).0, -22);
    assert!(d.t.is_none(), "nothing a refused caller sent was kept");
}

#[test]
fn an_upload_is_its_own_anchor_and_is_served_once_held() {
    let (_, parts, _) = honest();
    let c = parts.bytes();
    let want = anchor_of(&LISTING, &c).expect("a container");
    let mut d = Desk::default();
    let bodies = put_bodies(&LISTING, &c);
    for b in &bodies[..bodies.len() - 1] {
        assert_eq!(ask(&mut d, STORE, PUT, b), (0, Vec::new()));
    }
    assert_eq!(ask(&mut d, STORE, PUT, bodies.last().unwrap()), (0, want.to_vec()));
    assert_eq!(d.t.as_ref().map(|t| t.stage), Some(Stage::Uploading));
    assert_eq!(ask(&mut d, STORE, PUT, &bodies[0]).0, -16, "one transfer at a time");
    assert_eq!(ask(&mut d, INDEX, GET, &get_body(&LISTING, &want, 0)).0, -1202, "pending");
    let n = c.len() as u32;
    let up = [&[1][..], &n.to_le_bytes()].concat();
    assert_eq!(ask(&mut d, INDEX, STATUS, &want), (0, up), "uploading, its length known");
    d.finish(Ok(Vec::new()));
    assert_eq!(ask(&mut d, INDEX, STATUS, &want), (0, [&[3][..], &n.to_le_bytes()].concat()));
    assert_eq!(ask(&mut d, INDEX, GET, &get_body(&LISTING, &want, 0)), (0, c[..GET_MAX].to_vec()));
    assert_eq!(ask(&mut d, STORE, GET, &get_body(&LISTING, &want, n)), (0, Vec::new()));
    assert_eq!(ask(&mut d, STORE, GET, &get_body(&LISTING, &want, n + 1)).0, -22);
    assert_eq!(ask(&mut d, STORE, PUT, &bodies[0]), (0, Vec::new()), "a held one gives way");
}
