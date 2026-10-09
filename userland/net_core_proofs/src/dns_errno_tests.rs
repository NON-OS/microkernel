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

//! What a lookup that ended without an address tells its caller.

use crate::dns_verdict::{failure_errno, Found};
use crate::protocol::dns::{E_SERVFAIL, E_TIMEOUT};

// The browser (NO_UPSTREAM) and net.sockets (DNS_TIMEOUT) read 6 as a network
// with no working resolver; a lookup no server answered must say so, and one
// a server answered without an address must not.
#[test]
fn no_answer_at_all_is_a_timeout_and_a_bad_answer_is_not() {
    assert_eq!(E_TIMEOUT, 6);
    assert_eq!(failure_errno(Found::Pending), E_TIMEOUT);
    assert_eq!(failure_errno(Found::Failed), E_SERVFAIL);
}
