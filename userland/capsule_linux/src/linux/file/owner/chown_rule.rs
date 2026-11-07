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

/*
 * Whether chown finds the file it names: one of the person's folders is
 * the user-files broker's to answer, and the store is never asked for it,
 * as stat, access and statfs answer; any other path is the store's. Pure,
 * so the host proofs hold it.
 */
pub fn found<T>(broker: Option<Option<T>>, store: impl FnOnce() -> Option<T>) -> bool {
    match broker {
        Some(answer) => answer.is_some(),
        None => store().is_some(),
    }
}
