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

//! The session manager (security/policy/session/manager.rs) with the RNG
//! refusing: no user and no session is added, and a refused login leaves
//! the account as it was. A finding is pinned as the code has it.

use crate::security::policy::session::manager::SessionManager;
use crate::security::policy::session::types::PrivilegeLevel;
use crate::stand_in::rng;

pub(super) fn started() -> SessionManager {
    let m = SessionManager::new();
    rng::fail_from(None);
    m.init().expect("init");
    m
}

/// Finding: the uid is taken before the salt is drawn, so a refused create
/// spends one.
#[test]
fn a_user_is_not_created_with_no_randomness() {
    let m = started();
    rng::refuse();
    assert_eq!(m.create_user("eve", "pw", PrivilegeLevel::User), Err("no randomness"));
    assert!(m.get_user_by_name("eve").is_none());
    assert_eq!(m.user_count(), 0);
    rng::fail_from(None);
    assert_eq!(m.create_user("eve", "pw", PrivilegeLevel::User), Ok(1001));
    assert_eq!(rng::weak(), 0);
}

#[test]
fn a_user_with_no_password_is_refused_before_any_draw() {
    let m = started();
    rng::fail_from(None);
    let made = m.create_user("eve", "", PrivilegeLevel::User);
    assert_eq!(made, Err("an account needs a password"));
    assert_eq!((m.user_count(), rng::draws()), (0, 0));
}

#[test]
fn a_login_refused_for_its_token_starts_no_session_and_keeps_the_account() {
    let m = started();
    m.create_user("bob", "pw", PrivilegeLevel::User).expect("user");
    assert!(m.login("bob", "wrong").is_err());
    let before = m.get_user_by_name("bob").map(|u| (u.failed_attempts, u.last_login));
    rng::refuse();
    assert_eq!(m.login("bob", "pw"), Err("no randomness"));
    assert_eq!((m.session_count(), m.current().map(|s| s.id)), (0, None));
    assert_eq!(m.get_user_by_name("bob").map(|u| (u.failed_attempts, u.last_login)), before);
    rng::fail_from(None);
    let id = m.login("bob", "pw").expect("login");
    let s = m.current().expect("current");
    assert_eq!((s.id, s.token.to_vec()), (id, rng::given()[0].clone()));
    assert_eq!(m.get_user_by_name("bob").map(|u| u.failed_attempts), Some(0));
}
