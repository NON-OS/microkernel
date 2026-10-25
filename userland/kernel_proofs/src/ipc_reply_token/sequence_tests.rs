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

use super::model::Server;

/// One request the server never answers, then five calls, each replied to:
/// every reply carries its own call's token. Taking the oldest entry, the
/// second reply on carries the call before it, and the caller drops them all.
#[test]
fn one_swallowed_request_costs_no_later_reply() {
    let mut s = Server::default();
    s.call("c", 1);
    s.serve(1);
    for token in 2..=6 {
        s.call("c", token);
        s.serve(token);
        assert_eq!(s.reply("c"), Some(token), "call {token}");
    }
}

/// A server that takes several requests and answers them in order still
/// answers each caller's own call.
#[test]
fn a_pipelining_server_answers_each_call_in_order() {
    let mut s = Server::default();
    for (inbox, token) in [("c", 1), ("d", 2), ("e", 3)] {
        s.call(inbox, token);
        s.serve(token);
    }
    assert_eq!(s.reply("c"), Some(1));
    assert_eq!(s.reply("d"), Some(2));
    assert_eq!(s.reply("e"), Some(3));
    assert_eq!(s.reply("e"), None, "a second reply finds no call owed");
}
