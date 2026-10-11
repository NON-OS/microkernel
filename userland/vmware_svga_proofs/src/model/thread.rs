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

//! One adapter per test thread.

use std::cell::RefCell;

use super::svga::Svga;

thread_local! {
    static SVGA: RefCell<Option<Svga>> = const { RefCell::new(None) };
}

pub fn install(svga: Svga) {
    SVGA.with(|s| *s.borrow_mut() = Some(svga));
}

pub fn take() -> Svga {
    SVGA.with(|s| s.borrow_mut().take()).expect("no adapter installed on this thread")
}

pub fn with_svga<R>(f: impl FnOnce(&mut Svga) -> R) -> R {
    SVGA.with(|s| f(s.borrow_mut().as_mut().expect("no adapter installed on this thread")))
}
