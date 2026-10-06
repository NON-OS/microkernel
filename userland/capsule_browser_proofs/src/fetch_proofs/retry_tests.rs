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

//! Which failures are tried again, and which are not.

use super::deadline_tests::reading;
use crate::browser::fetch::budget::budget;
use crate::browser::fetch::deadline::due;
use crate::browser::fetch::retryable_error::retryable_error;
use crate::browser::fetch::security_error::security_error;
use crate::browser::net::mixnet::Network;

/* fail() retries only what retryable_error accepts and security_error does not. */
fn retried(msg: &str) -> bool {
    retryable_error(msg) && !security_error(msg)
}

#[test]
fn nothing_heard_timed_out_and_is_retried() {
    assert_eq!(due(&reading(0, 0), budget(Network::Direct), 12_001), Some("timed out"));
    assert!(retried("timed out"));
    assert!(retried("connect failed"), "a connect that failed received nothing");
}

#[test]
fn heard_then_stopped_is_stalled_and_not_retried() {
    assert!(!retried("stalled"), "trying again only repeats the wait");
}

#[test]
fn a_verification_failure_is_never_retried() {
    assert!(!retried("tls handshake refused"));
    assert!(!retried("tls handshake failed"));
    assert!(!retried("tls record failed"));
}
