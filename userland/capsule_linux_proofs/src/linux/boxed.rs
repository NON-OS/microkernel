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

//! The capsule's armed box, as the mounted files reach it: the real kinds,
//! and a box held on this test thread, so a mounted refusal site's note is
//! seen where a test can read it.

use std::cell::RefCell;

pub use crate::boxed::kind::{of_needs, Kind};
use crate::boxed::tally::Tally;

thread_local! {
    pub static TALLY: RefCell<Tally> = RefCell::new(Tally::default());
    pub static FOLDERS: RefCell<Option<Vec<Vec<u8>>>> = const { RefCell::new(None) };
    /// Each write let through, as a learn run records it.
    pub static WROTE: RefCell<Vec<Vec<u8>>> = const { RefCell::new(Vec::new()) };
}

pub fn note(kind: Kind, example: &[u8]) {
    TALLY.with(|t| t.borrow_mut().note(kind, example));
}

pub fn may_write(path: &[u8]) -> bool {
    FOLDERS.with(|f| {
        f.borrow().as_ref().is_none_or(|folders| crate::boxed::folders::may_write(folders, path))
    })
}

pub fn wrote(path: &[u8]) {
    WROTE.with(|w| w.borrow_mut().push(path.to_vec()));
}
