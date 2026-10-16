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

//! The router the shell believes (state/input_source_rule.rs): the pid the
//! registry named, asked again only when a frame comes from another pid,
//! so a restarted router is believed at once.

use core::sync::atomic::{AtomicU32, Ordering};

use crate::server::handlers::launcher_request::lookup_pid;
use crate::state::input_source_rule::Router;

const ROUTER: &[u8] = b"input_router";
static BELIEVED: AtomicU32 = AtomicU32::new(0);

pub fn is_router(sender: u32) -> bool {
    let now = Router::from_word(BELIEVED.load(Ordering::Relaxed));
    if now.believes(sender) {
        return true;
    }
    let next = now.after_lookup(lookup_pid(ROUTER));
    BELIEVED.store(next.word(), Ordering::Relaxed);
    next.believes(sender)
}
