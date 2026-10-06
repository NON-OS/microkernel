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

//! Contacts that can only leave directly: the time server net.ntp asks at
//! boot, the Terminal's ping and its lookups in the clear. Each is made only
//! when Direct is the default, which is exactly when `pick` goes direct.

use nonos_policy_proto::route::{ANYONE, DIRECT, NYM};

use crate::direct_only::direct_refused;
use crate::pick::{pick, Route};

#[test]
fn only_direct_lets_a_direct_contact_through() {
    let defaults = core::iter::once(None).chain((0..=u8::MAX).map(Some));
    for default in defaults {
        let allowed = direct_refused(default).is_none();
        assert_eq!(allowed, default == Some(DIRECT), "{default:?}");
        /* The same answer pick gives, whichever networks are running. */
        for (nym, anon) in [(0, 0), (7, 0), (0, 9), (7, 9)] {
            assert_eq!(allowed, pick(default, nym, anon) == Route::Direct, "{default:?}");
        }
    }
}

#[test]
fn time_sync_is_skipped_unless_direct_and_says_why() {
    /* The NTP decision at boot, for each default the store can hold. */
    assert_eq!(direct_refused(Some(DIRECT)), None);
    let nym = direct_refused(Some(NYM)).unwrap_or_default();
    assert!(nym.contains("Nym mixnet") && nym.contains("anonymous"), "{nym}");
    let anyone = direct_refused(Some(ANYONE)).unwrap_or_default();
    assert!(anyone.contains("Anyone") && anyone.contains("anonymous"), "{anyone}");
    /* Run before the store is up: unreadable is the default, the mixnet. */
    let unread = direct_refused(None).unwrap_or_default();
    assert!(unread.contains("could not be read") && unread.contains("Nym mixnet"), "{unread}");
    let unknown = direct_refused(Some(200)).unwrap_or_default();
    assert!(unknown.contains("Nym mixnet"), "{unknown}");
}

#[test]
fn every_refusal_is_one_plain_line() {
    let defaults = core::iter::once(None).chain((0..=u8::MAX).map(Some));
    for why in defaults.filter_map(direct_refused) {
        assert!(!why.is_empty() && why.is_ascii() && !why.contains('\n'), "{why}");
    }
}
