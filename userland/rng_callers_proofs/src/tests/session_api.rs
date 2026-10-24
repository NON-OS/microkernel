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

//! The session system's start (security/policy/session/{manager,api}.rs):
//! it makes no account and no session, so it draws nothing and starts with
//! no randomness; the anonymous login has no account to log in to.

use super::session_manager::started;
use crate::security::policy::session::api::{init, session_manager};
use crate::stand_in::{kernel, rng};

/// The one test that touches the global manager.
#[test]
fn the_session_system_starts_empty_and_draws_nothing() {
    kernel::lines();
    rng::refuse();
    assert_eq!(init(), Ok(()));
    assert_eq!((session_manager().user_count(), session_manager().session_count()), (0, 0));
    assert_eq!((rng::draws(), rng::weak()), (0, 0));
    assert_eq!(kernel::lines().len(), 1);
}

#[test]
fn the_anonymous_login_gives_0_and_draws_nothing() {
    let m = started();
    rng::fail_from(None);
    assert_eq!(m.login_anonymous(), 0);
    assert_eq!((m.session_count(), m.current().map(|s| s.id)), (0, None));
    assert_eq!(rng::draws(), 0);
}
