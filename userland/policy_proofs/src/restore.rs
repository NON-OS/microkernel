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

//! `crate::restore` as the get path asks it: the real rule of which reads
//! wait, over a settled flag each test sets for its own thread.

use std::cell::Cell;

use nonos_policy_proto::Field;

#[path = "../../capsule_policy/src/restore/awaits.rs"]
pub mod awaits;
#[path = "../../capsule_policy/src/restore/stage.rs"]
pub mod stage;

thread_local! {
    static SETTLED: Cell<bool> = const { Cell::new(true) };
}

/// Set whether the restore has finished, for this thread's requests.
pub fn set_settled(settled: bool) {
    SETTLED.with(|s| s.set(settled));
}

pub fn awaits_restore(field: Field) -> bool {
    awaits::awaits(field, SETTLED.with(|s| s.get()))
}
