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

//! The W^X and NX-stack policy switches.

use core::sync::atomic::{AtomicBool, Ordering};

static ENFORCE_WX: AtomicBool = AtomicBool::new(true);
static ENFORCE_NX_STACK: AtomicBool = AtomicBool::new(true);

pub fn enforce_wx_policy() -> bool {
    ENFORCE_WX.load(Ordering::Relaxed)
}
pub fn enforce_nx_stack() -> bool {
    ENFORCE_NX_STACK.load(Ordering::Relaxed)
}
pub fn set_wx_policy(enforce: bool) {
    ENFORCE_WX.store(enforce, Ordering::Relaxed);
}
pub fn set_nx_stack_policy(enforce: bool) {
    ENFORCE_NX_STACK.store(enforce, Ordering::Relaxed);
}
