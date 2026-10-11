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

//! PUT chunks out of order, from another caller, or that are no container
//! are refused; a download is kept only when it is the anchor asked for.

use nonos_proof_abi::anchor_of;
use nonos_proof_abi::wire::fetch::{get_body, put_bodies, Put, Stage, GET, PUT, STATUS};

use super::fetch_desk_tests::{ask, INDEX, STORE};
use crate::fetch_desk::{Caller, Desk};
use crate::fixture::{honest, LISTING};

const OTHER: Caller = Caller { pid: 70, store: true, index: false };

#[test]
fn chunks_come_in_order_from_one_caller_and_make_a_container() {
    let c = vec![5u8; 70_000];
    let bodies = put_bodies(&LISTING, &c);
    let mut d = Desk::default();
    assert_eq!(ask(&mut d, STORE, PUT, &bodies[1]).0, -22, "not opened");
    assert_eq!(ask(&mut d, STORE, PUT, &bodies[0]).0, 0);
    assert_eq!(ask(&mut d, OTHER, PUT, &bodies[0]).0, -16, "another caller's");
    assert_eq!(ask(&mut d, OTHER, PUT, &bodies[1]).0, -16);
    let skip = Put { listing_id: &LISTING, total: 70_000, offset: 1, chunk: &[5] }.encode();
    assert_eq!(ask(&mut d, STORE, PUT, &skip).0, -22, "out of order");
    assert_eq!(ask(&mut d, STORE, PUT, &bodies[0]).0, 0, "its own caller starts again");
    assert_eq!(ask(&mut d, STORE, PUT, &bodies[1]).0, -1205, "not a container");
    assert_eq!(d.t.as_ref().map(|t| (t.stage, t.bytes.len())), Some((Stage::Failed, 0)));
    assert_eq!(ask(&mut d, OTHER, PUT, &bodies[0]).0, 0, "a failed one gives way");
    d.abandon();
    assert!(d.t.is_none(), "an unfinished PUT is dropped");
}

#[test]
fn a_download_is_kept_only_as_the_anchor_asked_for() {
    let (_, parts, _) = honest();
    let c = parts.bytes();
    let want = anchor_of(&LISTING, &c).expect("a container");
    let get = get_body(&LISTING, &want, 0);
    let mut d = Desk::default();
    assert_eq!(ask(&mut d, INDEX, GET, &get).0, -1202, "not held: its download starts");
    assert_eq!(d.t.as_ref().map(|t| t.stage), Some(Stage::Downloading));
    assert_eq!(ask(&mut d, STORE, GET, &get_body(&LISTING, &[1; 32], 0)).0, -16);
    let mut other = c.clone();
    let n = other.len();
    other[n - 1] ^= 1;
    d.finish(Ok(other));
    assert_eq!(ask(&mut d, INDEX, STATUS, &want), (-1204, vec![4, 0, 0, 0, 0]));
    assert_eq!(ask(&mut d, INDEX, GET, &get).0, -1202, "asked again after a failure");
    d.finish(Err(-1206));
    assert_eq!(ask(&mut d, INDEX, STATUS, &want).0, -1206);
    assert_eq!(ask(&mut d, INDEX, GET, &get).0, -1202);
    d.finish(Ok(c.clone()));
    assert_eq!(ask(&mut d, INDEX, GET, &get_body(&LISTING, &want, 65_536)).0, 0);
    let elsewhere = get_body(&[0x77; 32], &want, 0);
    assert_eq!(ask(&mut d, INDEX, GET, &elsewhere).0, -1202, "another listing is another ask");
}
