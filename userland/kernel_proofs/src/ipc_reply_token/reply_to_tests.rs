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

fn to(inbox: &str, token: u64) -> Option<(String, u64)> {
    Some((inbox.to_string(), token))
}

/// A server takes three requests, then answers them out of order: each
/// answer reaches its own caller with its own call's token.
#[test]
fn a_pipelining_server_answers_out_of_order_each_to_its_caller() {
    let mut s = Server::default();
    let ids: Vec<u64> = [("c", 11), ("d", 12), ("e", 13)]
        .into_iter()
        .map(|(inbox, token)| {
            s.call(inbox, token);
            s.serve(token)
        })
        .collect();
    assert_eq!(ids, [1, 2, 3], "ids are the server's own, not the tokens");
    assert_eq!(s.reply_to(3), to("e", 13));
    assert_eq!(s.reply_to(1), to("c", 11));
    assert_eq!(s.reply_to(2), to("d", 12));
}

/// A second answer reaches no one, even beside the caller's other dequeued call.
#[test]
fn a_second_reply_to_reaches_no_one_beside_another_dequeued_call() {
    let mut s = Server::default();
    s.call("c", 11);
    s.call("c", 12);
    let (first, second) = (s.serve(11), s.serve(12));
    assert_eq!(s.reply_to(first), to("c", 11));
    assert_eq!(s.reply_to(first), None, "request {first} is answered");
    assert_eq!(s.reply_to(second), to("c", 12));
}

/// A request still in the inbox, one never queued, and 0 answer nothing.
#[test]
fn an_undequeued_or_unknown_request_is_never_answered() {
    let mut s = Server::default();
    s.call("c", 11);
    assert_eq!(s.reply_to(1), None, "still in the inbox");
    assert_eq!(s.reply_to(7), None);
    assert_eq!(s.serve(0), 0, "a kernel sender's request is owed nothing");
    assert_eq!(s.reply_to(0), None);
    assert_eq!(s.serve(11), 1);
    assert_eq!(s.reply_to(1), to("c", 11));
}

/// A request answered by name is spent for the other reply paths too.
#[test]
fn a_request_answered_by_name_is_not_answered_again_by_mk_ipc_reply() {
    let mut s = Server::default();
    s.call("c", 11);
    let id = s.serve(11);
    assert_eq!(s.reply_to(id), to("c", 11));
    assert_eq!(s.reply("c"), None);
    assert_eq!(s.redirect(), None);
}
