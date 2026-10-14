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

//! One reading of a refresh into the value it fills. Pure, so wallet_proofs
//! holds the rule: what came replaces what was shown, and what did not come
//! is shown as not read, never as the last value it had.

/// Take `read` into `value`; true when what is shown changed.
pub fn take<T: PartialEq + Copy>(read: Option<T>, value: &mut T, ready: &mut bool) -> bool {
    match read {
        Some(v) => {
            let changed = !*ready || v != *value;
            *value = v;
            *ready = true;
            changed
        }
        None => core::mem::replace(ready, false),
    }
}
