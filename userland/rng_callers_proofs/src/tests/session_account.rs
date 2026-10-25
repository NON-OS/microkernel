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

//! A user's password salt (security/policy/session/account.rs): drawn only
//! from the seeded RNG. The salt is private; a test reads it from the
//! account's debug form, which shows it.

use crate::security::policy::session::account::UserAccount;
use crate::stand_in::rng;

fn salt_shown(given: &[u8]) -> String {
    format!("password_salt: {given:?}")
}

#[test]
fn an_account_refuses_with_no_randomness() {
    rng::refuse();
    assert_eq!(UserAccount::new(7, 7, "u", "pw").err(), Some("no randomness"));
    assert_eq!(rng::weak(), 0);
}

#[test]
fn accounts_hash_their_password_under_distinct_drawn_salts() {
    rng::fail_from(None);
    let a = UserAccount::new(7, 7, "u", "pw").expect("account");
    let b = UserAccount::new(7, 7, "u", "pw").expect("account");
    let given = rng::given();
    assert!(format!("{a:?}").contains(&salt_shown(&given[0])));
    assert!(format!("{b:?}").contains(&salt_shown(&given[1])));
    assert_ne!(format!("{a:?}"), format!("{b:?}"));
    assert!(a.verify_password("pw") && b.verify_password("pw"));
    assert!(!a.verify_password("px"));
}

#[test]
fn a_password_change_refused_keeps_the_salt_and_the_old_password() {
    rng::fail_from(None);
    let mut a = UserAccount::new(7, 7, "u", "old").expect("account");
    let before = format!("{a:?}");
    rng::refuse();
    assert_eq!(a.change_password("new"), Err("no randomness"));
    assert_eq!(format!("{a:?}"), before);
    assert!(a.verify_password("old") && !a.verify_password("new"));
    rng::fail_from(None);
    a.change_password("new").expect("changed");
    assert!(format!("{a:?}").contains(&salt_shown(&rng::given()[0])));
    assert!(a.verify_password("new") && !a.verify_password("old"));
}

#[test]
fn an_empty_password_is_refused_before_any_draw() {
    rng::fail_from(None);
    assert_eq!(UserAccount::new(7, 7, "u", "").err(), Some("an account needs a password"));
    let mut a = UserAccount::new(7, 7, "u", "pw").expect("account");
    let before = format!("{a:?}");
    assert_eq!(a.change_password(""), Err("an account needs a password"));
    assert_eq!((format!("{a:?}"), rng::draws()), (before, 1));
}
