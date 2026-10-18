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

//! nonos.prove.fetch's wire: its own magic, and fixed GET and STATUS bodies.

use nonos_proof_abi::wire::fetch::{
    anchor_of_body, get_body, get_of, status_body, Stage, FETCH_MAGIC,
};
use nonos_proof_abi::wire::{request, Header, PROVE_MAGIC};

#[test]
fn fetch_frames_are_not_prove_frames() {
    let tx = request(FETCH_MAGIC, 1, 3, &[1; 4]);
    assert_eq!(&tx[..4], b"NPFT");
    assert!(Header::parse(&tx, PROVE_MAGIC).is_none());
    assert!(Header::parse(&tx, FETCH_MAGIC).is_some());
}

#[test]
fn get_and_status_bodies_have_one_length() {
    let b = get_body(&[1; 32], &[2; 32], 70_000);
    assert_eq!(get_of(&b), Some(([1; 32], [2; 32], 70_000)));
    assert_eq!(get_of(&b[..67]), None);
    assert_eq!(get_of(&[&b[..], &[0]].concat()), None);
    assert_eq!(anchor_of_body(&[3; 32]), Some([3; 32]));
    assert_eq!(anchor_of_body(&[3; 33]), None);
    assert_eq!(status_body(Stage::Held, 9), [3, 9, 0, 0, 0]);
    let stages = (0..=5).map(Stage::from_u8).collect::<Vec<_>>();
    assert_eq!(stages[3], Some(Stage::Held));
    assert_eq!((stages[4], stages[5]), (Some(Stage::Failed), None));
}
