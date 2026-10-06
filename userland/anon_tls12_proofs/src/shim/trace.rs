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


//! The serial log, kept per thread so a test can read what was said.

use std::cell::RefCell;
use std::string::String;
use std::vec::Vec;

thread_local! {
    static LOG: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

pub fn say(stage: &[u8]) {
    LOG.with(|l| l.borrow_mut().push(String::from_utf8_lossy(stage).into_owned()));
}

pub fn take() -> Vec<String> {
    LOG.with(|l| core::mem::take(&mut *l.borrow_mut()))
}
