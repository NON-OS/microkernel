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

//! Spends the service keeps that this window never saw proved.

use alloc::string::String;

use crate::wallet::state::shield_log::Kind;
use crate::wallet::state::shield_ui::{ShieldUi, Spend};

/* Each one the service holds older than this window's own spends, put
 * first in order and followed as a payment it did not see proved. */
pub(super) fn insert_older(ui: &mut ShieldUi, older: alloc::vec::Vec<&str>) {
    for (at, id) in older.into_iter().enumerate() {
        ui.earlier.insert(
            at,
            Spend {
                id: Some(String::from(id)),
                kind: Kind::Send,
                asset: 0,
                amount: String::new(),
                published: true,
                state: String::new(),
                minutes: 0,
                tx: None,
                self_settle: false,
                refusal: None,
                weakened: alloc::vec::Vec::new(),
            },
        );
    }
}
