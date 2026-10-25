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

//! Session keys carry frames both ways once AUTH_OK is in.

use crate::kit::pair::Pair;
use nsh_client::Event;
use nsh_wire::Kind;

#[test]
fn session_keys_carry_frames_both_ways() {
    let mut p = Pair::good();
    p.login();
    p.client.input(b"ls\n").unwrap();
    assert_eq!(p.sent(), vec![(Kind::Input.byte(), b"ls\n".to_vec())]);
    let out = p.server.seal(Kind::Output, b"bin etc\n");
    p.feed(&out).unwrap();
    assert_eq!(p.client.events(), vec![Event::Output(b"bin etc\n".to_vec())]);
}
