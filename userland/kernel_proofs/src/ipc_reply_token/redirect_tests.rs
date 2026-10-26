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

/// A redirect reply answers the request the server dequeued last, whoever
/// called first, and only once.
#[test]
fn a_redirect_answers_the_last_dequeued_call_once() {
    let mut s = Server::default();
    s.call("c", 1);
    s.call("d", 2);
    s.serve(2);
    assert_eq!(s.redirect(), Some(("d".to_string(), 2)), "not the front, c's");
    assert_eq!(s.redirect(), None, "a second reply to call 2");
    s.serve(1);
    assert_eq!(s.redirect(), Some(("c".to_string(), 1)));
}

/// A request a kernel sender made carries token 0 and is owed no call: the
/// server's reply to it reaches no capsule, and leaves every call owed.
#[test]
fn a_redirect_to_a_kernel_request_reaches_no_one() {
    let mut s = Server::default();
    s.call("c", 1);
    s.serve(0);
    assert_eq!(s.redirect(), None);
    s.serve(1);
    assert_eq!(s.redirect(), Some(("c".to_string(), 1)));
}

/// A dequeue marks only the call it took: an undequeued call stays out of
/// reach of a direct reply and a redirect alike.
#[test]
fn a_dequeue_marks_its_own_call_only() {
    let mut s = Server::default();
    s.call("c", 1);
    s.call("c", 2);
    s.serve(2);
    assert_eq!(s.reply("c"), Some(2));
    assert_eq!(s.reply("c"), None, "call 1 is still in the inbox");
    assert_eq!(s.redirect(), None);
    s.serve(1);
    assert_eq!(s.reply("c"), Some(1));
}
