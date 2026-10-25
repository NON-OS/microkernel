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

//! Session bodies the server would refuse are not sent and do not end
//! the session; one escalation runs at a time; nothing follows CLOSE.

use crate::kit::grant;
use crate::kit::pair::Pair;
use nsh_client::Refusal;
use nsh_wire::Kind;

#[test]
fn one_escalation_at_a_time_and_nothing_after_close() {
    let mut p = Pair::good();
    p.login();
    p.client.escalate(&grant::escalation()).unwrap();
    assert_eq!(p.client.escalate(&grant::escalation()), Err(Refusal::NotNow));
    assert_eq!(p.sent().len(), 1);
    p.client.close().unwrap();
    assert_eq!(p.sent(), vec![(Kind::Close.byte(), vec![])]);
    assert_eq!(p.client.input(b"x"), Err(Refusal::NotNow));
    assert_eq!(p.client.close(), Err(Refusal::NotNow));
}
