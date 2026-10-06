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

//! W on the Wi-Fi page: the same switch the row's toggle flips, through the
//! same store write, so the key and the click cannot disagree.

use nonos_policy_proto::Field;

use crate::settings::state::{cached_value, FieldValue, State};

use super::commit_bool::commit_bool;

pub(super) fn toggle_radio(state: &mut State) {
    // An unread value counts as on, as the scan and join treat it.
    let on = !matches!(cached_value(state, Field::WifiRadio), FieldValue::Bool(false));
    commit_bool(state, Field::WifiRadio, !on);
}
