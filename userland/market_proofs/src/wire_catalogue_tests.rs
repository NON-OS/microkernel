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

//! The catalogue: every listing read back, and a cut or overlong one refused.

use super::*;

fn catalogue() -> Vec<u8> {
    let mut body = 2u32.to_le_bytes().to_vec();
    lp(&mut body, b"linux.htop");
    body.extend_from_slice(&[7; 32]);
    lp(&mut body, b"htop");
    body.push(1);
    lp(&mut body, b"nonos.app.notes");
    body.extend_from_slice(&[8; 32]);
    lp(&mut body, b"Notes");
    body.push(0);
    body
}

#[test]
fn the_catalogue_reads_back_every_listing() {
    let list = parse_list(&catalogue()).unwrap();
    assert_eq!(list.len(), 2);
    assert_eq!(list[0].id, b"linux.htop");
    assert_eq!(list[0].measurement, [7; 32]);
    assert_eq!(list[0].name, b"htop");
    assert!(list[0].ready);
    assert_eq!(list[1].name, b"Notes");
    assert!(!list[1].ready);
}

#[test]
fn a_catalogue_cut_anywhere_is_refused_not_misread() {
    let body = catalogue();
    for n in 0..body.len() {
        assert!(parse_list(&body[..n]).is_none(), "cut at {n}");
    }
}

#[test]
fn a_catalogue_naming_more_listings_than_it_holds_is_refused() {
    let mut body = catalogue();
    body[0..4].copy_from_slice(&u32::MAX.to_le_bytes());
    assert!(parse_list(&body).is_none());
}
