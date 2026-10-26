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

//! The tray table's `Default` (state/tray/table.rs) is the empty table the
//! const `new` makes.

#[cfg(test)]
mod tests {
    use super::super::tray::{TrayEntry, TrayTable};

    #[test]
    fn the_default_table_is_the_empty_one_new_makes() {
        let mut t = TrayTable::default();
        assert!(t.find(7, 1).is_none());
        let e = TrayEntry { owner_pid: 7, tray_id: 1, in_use: true, ..TrayEntry::default() };
        assert!(t.insert(e).is_ok() && t.insert(e).is_err(), "taken once, as from new");
    }
}
