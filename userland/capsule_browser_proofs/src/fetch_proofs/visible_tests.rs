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

//! A tick that moves nothing has nothing new to draw.

use super::fetch_fixtures::{step, url_of};
use super::fetch_wire::FakeWire;
use crate::browser::fetch::progress::status;
use crate::browser::fetch::types::{Fetch, Phase};

#[test]
fn a_step_that_moves_nothing_changes_nothing_drawn() {
    let mut w = FakeWire::at(0);
    let mut f = Fetch::new(url_of("http://10.0.2.2/"), 11, Phase::ReadBody, 0);
    let shown = status(&f);
    assert_eq!(shown, "Downloading 10.0.2.2");
    for _ in 0..3 {
        w.advance(50);
        assert!(!step(&mut w, &mut f), "no bytes, no move");
        assert_eq!(status(&f), shown);
    }
    w.deliver(11, b"HTTP/1.1 200 OK\r\nContent-Length: 4\r\n\r\nab");
    assert!(!step(&mut w, &mut f), "bytes, but the phase stands");
    assert_eq!(status(&f), shown, "the same line: no repaint for it");
    w.deliver(11, b"cd");
    assert!(step(&mut w, &mut f), "the response is whole");
    assert_ne!(status(&f), shown);
}

#[test]
fn a_connect_shows_its_host_at_once() {
    let mut w = FakeWire::at(0);
    let f = crate::browser::fetch::open::open(&mut w, url_of("http://10.0.2.99/"), None);
    assert_eq!(status(&f.expect("open")), "Connecting to 10.0.2.99");
}
