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

//! OP_GET_LOGO's body: what the market writes the store reads back, and a
//! body that is cut, padded or not square is no logo at all.

use nonos_market_proto::logo::{logo_body, logo_side, parse_logo, LOGO_MAX};
use nonos_market_proto::{OP_GET_LINUX_POLICY, OP_GET_LOGO};

use super::table::embedded;

#[test]
fn the_op_follows_the_linux_policy() {
    assert_eq!((OP_GET_LINUX_POLICY, OP_GET_LOGO), (8, 9));
}

#[test]
fn every_carried_logo_reads_back_as_served() {
    for id in ["linux.git", "linux.vim", "linux.profanity"] {
        let bytes = embedded(id).expect("carried");
        let side = logo_side(bytes.len()).expect("square");
        let body = logo_body(Some(bytes)).expect("fits");
        let logo = parse_logo(&body).expect("reads").expect("a logo");
        assert_eq!((logo.side as usize, logo.rgba.as_slice()), (side, bytes), "{id}");
    }
    assert!(matches!(parse_logo(&logo_body(None).expect("none")), Some(None)));
}

#[test]
fn a_body_that_is_not_one_logo_is_refused() {
    let git = logo_body(embedded("linux.git")).expect("fits");
    assert!(parse_logo(&git[..git.len() - 1]).is_none(), "cut");
    assert!(parse_logo(&[git.as_slice(), &[0]].concat()).is_none(), "padded");
    assert!(parse_logo(&[0, 0]).is_none(), "none, with bytes");
    assert!(parse_logo(&[]).is_none(), "empty");
    let mut wrong = git.clone();
    wrong[0] = 35;
    assert!(parse_logo(&wrong).is_none(), "a side the bytes do not have");
    assert_eq!(logo_side(4 * LOGO_MAX * LOGO_MAX), Some(LOGO_MAX));
    assert!(logo_body(Some(&[0; 4 * 49 * 49])).is_none());
}
