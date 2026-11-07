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

//! chown finds a file in the person's folders as stat does (file/owner
//! chown_rule.rs): the user-files broker answers for it and the store is
//! not asked. LibreOffice replaces a document it saves and then gives it
//! its old owner back (osl_replaceFile: stat, rename, chown), so a chown
//! that said ENOENT for a file stat had just found failed every save into
//! Documents.

use core::cell::Cell;

use crate::chown_rule::found;

/// `found` with the broker's answer `broker` and a store that holds the
/// file when `held`, and whether the store was asked.
fn ask(broker: Option<Option<u64>>, held: bool) -> (bool, bool) {
    let asked = Cell::new(false);
    let got = found(broker, || {
        asked.set(true);
        held.then_some(0)
    });
    (got, asked.get())
}

#[test]
fn a_file_the_broker_holds_is_found_and_the_store_is_not_asked() {
    assert_eq!(ask(Some(Some(4096)), false), (true, false));
    assert_eq!(ask(Some(Some(0)), true), (true, false));
}

#[test]
fn a_name_the_broker_has_not_is_missing_whatever_the_store_holds() {
    assert_eq!(ask(Some(None), true), (false, false));
    assert_eq!(ask(Some(None), false), (false, false));
}

#[test]
fn any_other_path_is_the_stores() {
    assert_eq!(ask(None, true), (true, true));
    assert_eq!(ask(None, false), (false, true));
}
