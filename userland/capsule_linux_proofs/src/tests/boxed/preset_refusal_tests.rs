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

//! Every box the command line may not name, and a list held once.

use crate::boxed::preset::parse;

#[test]
fn every_malformed_box_is_refused() {
    for bad in [
        &b""[..],
        b"lenient",
        b"allow:",
        b"allow:tmp/a",
        b"allow:/tmp/",
        b"allow:/tmp//a",
        b"allow:/tmp/../etc",
        b"allow:/tmp/./a",
        b"allow:/etc",
        b"allow:/usr/bin",
        b"allow:net:example.com",
        b"allow:net:example.com:0",
        b"allow:net:example.com:65536",
        b"allow:net:1.2.3.4:x",
        b"allow:net:-bad-.com:443",
        b"allow:net::443",
        b"allow:net:1.2.3:443",
        b"allow:/tmp/a b",
        b"allow:/tmp/\x1b[2J",
        b"allow:/tmp/caf\xc3\xa9",
    ] {
        assert!(parse(bad).is_err(), "{} was taken", String::from_utf8_lossy(bad));
    }
}

#[test]
fn more_than_sixty_four_destinations_are_refused() {
    let mut word = b"allow:".to_vec();
    for p in 1..=65u16 {
        word.extend_from_slice(format!("net:10.0.0.1:{p},").as_bytes());
    }
    word.pop();
    assert!(parse(&word).is_err());
    let at_most: Vec<u8> = word[..word.len() - ",net:10.0.0.1:65".len()].to_vec();
    assert!(parse(&at_most).is_ok());
}

#[test]
fn a_destination_named_twice_is_held_once() {
    let p = parse(b"allow:net:10.0.0.1:80,net:10.0.0.1:80,net:A.com:1,net:a.com:1").unwrap();
    assert_eq!(nonos_linux_policy::dests::check(&p.dests), Ok(2));
}
