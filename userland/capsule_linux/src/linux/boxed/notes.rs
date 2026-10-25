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

//! What the serve loop tells the armed box as the run goes: whether a write
//! is let through, a new call, each refusal, and each destination reached.

use super::kind::Kind;
use super::state::BOX;

pub fn may_write(path: &[u8]) -> bool {
    BOX.0.borrow().as_ref().is_none_or(|a| super::folders::may_write(&a.policy.folders, path))
}

pub fn call_begins() {
    if let Some(a) = BOX.0.borrow_mut().as_mut() {
        a.tally.call_begins();
    }
}

pub fn note(kind: Kind, example: &[u8]) {
    if let Some(a) = BOX.0.borrow_mut().as_mut() {
        a.tally.note(kind, example);
    }
}

pub fn connected(dest: &[u8]) {
    if let Some(a) = BOX.0.borrow_mut().as_mut() {
        a.connected.note(dest);
    }
}
