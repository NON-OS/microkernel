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

/// A server answers one request twice. The second reply finds no call
/// dequeued and unanswered, even with the caller's next call waiting in the
/// inbox, so it reaches no one; the next call is answered when it is taken.
#[test]
fn a_second_reply_to_an_answered_request_reaches_no_one() {
    let mut s = Server::default();
    s.call("c", 1);
    s.serve(1);
    assert_eq!(s.reply("c"), Some(1));
    s.call("c", 2);
    assert_eq!(s.reply("c"), None, "call 2 is still in the inbox");
    s.serve(2);
    assert_eq!(s.reply("c"), Some(2));
}
