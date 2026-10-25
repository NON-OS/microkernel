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

//! A session's token (security/policy/session/session.rs): drawn only from
//! the seeded RNG; with none, no session is made.

use crate::security::policy::session::account::UserAccount;
use crate::security::policy::session::session::UserSession;
use crate::stand_in::rng;

#[test]
fn a_session_refuses_with_no_randomness() {
    rng::fail_from(None);
    let user = UserAccount::new(7, 7, "u", "pw").expect("account");
    rng::refuse();
    assert_eq!(UserSession::new(1, &user).err(), Some("no randomness"));
    assert_eq!(rng::weak(), 0);
}

#[test]
fn each_session_token_is_drawn_fresh() {
    rng::fail_from(None);
    let user = UserAccount::new(7, 7, "u", "pw").expect("account");
    rng::fail_from(None);
    let a = UserSession::new(1, &user).expect("session");
    let b = UserSession::new(2, &user).expect("session");
    assert_eq!(rng::given(), [a.token.to_vec(), b.token.to_vec()]);
    assert_ne!(a.token, b.token);
}
