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

//! The reference and the two write ops the sweep drives the store with.

use super::kit::{level, record};
use crate::faulty::Faulty;
use nonos_bt_bond::{KeysError, Store};

pub(super) const N: usize = 3;
/// Each identity's last written level, or None when forgotten or never kept.
pub(super) type Model = [Option<bool>; N];

/// Every step: a present identity is one the owner wrote and has not
/// forgotten — never a phantom. At a reopen the durable level settles at
/// exactly the last written, so it never exceeds it and a forgotten identity
/// is gone. (Between reopens the live book may still hold the prior bond
/// across a re-pair the keyring refused to finish; the durable state is the
/// one proved exact.)
pub(super) fn check(s: &Store<Faulty>, m: &Model, reopened: bool) {
    for (i, &w) in m.iter().enumerate() {
        match (level(s, i), w) {
            (Some(_), None) => panic!("identity {i} present but forgotten or never written"),
            (Some(b), Some(w)) if reopened => {
                assert_eq!(b, w, "reopen did not settle identity {i} at its last written level");
            }
            _ => {}
        }
    }
}

/// One keep of identity `i`, with a keyring fault chosen by `fault`.
pub(super) fn keep(s: &mut Store<Faulty>, m: &mut Model, i: usize, auth: bool, fault: usize) {
    match fault {
        1 => s.keys().store = Some(KeysError::Full),
        2 => s.keys().delete = Some(KeysError::Denied),
        _ => {}
    }
    let _ = s.keep(record(i, auth));
    (s.keys().store, s.keys().delete) = (None, None);
    // The store runs before any delete, so unless the store itself was
    // refused the new record is in the keyring and wins the next open.
    if fault != 1 {
        m[i] = Some(auth);
    }
}
