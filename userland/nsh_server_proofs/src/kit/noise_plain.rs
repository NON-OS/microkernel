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

//! The server fed bytes before anything has arrived.

use super::raw_run::RawRun;
use nsh_wire::Kind;

/// Before anything: nothing, `ABORT(VERSION)`, or the hellos' answer.
pub(super) fn plain(rest: &[u8]) {
    let mut r = RawRun::new();
    let out = r.send(rest);
    let version = out == [3, 0, 0, 0, 0x7F, 2, 0];
    assert!(out.is_empty() || version || out.get(4) == Some(&Kind::ServerHello.byte()), "{out:?}");
    if r.server.is_over() {
        assert!(r.send(&[1, 0, 0, 0, 1]).is_empty(), "nothing after the end");
    }
}
