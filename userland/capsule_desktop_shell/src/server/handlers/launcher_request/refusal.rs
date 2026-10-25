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

//! Why a launch spawned nothing: an app that opens one window only, or the
//! kernel's refusal, taken once.

use core::sync::atomic::{AtomicI64, Ordering};

use crate::state::says::NOTHING_REFUSED;

// Services that run as one window: a second would fight the first over the
// same session rather than give the user anything new. The App Store holds
// the catalog and every install in flight, and its capsule declares no
// instance endpoints, so a spawn could only fail and say so.
const SINGLE_INSTANCE: [&[u8]; 1] = [b"app.store"];

/// Why the last launch spawned nothing, taken once: the kernel's answer to
/// the spawn it refused, `NOT_ASKED` for an app that opens one window only,
/// or `NOTHING_REFUSED` when no spawn was asked since (a menu raising a
/// window that turned out gone). What a launch that opened nothing says
/// (`state::says::not_opened`), never an answer left from an earlier click.
pub fn take_refusal() -> i64 {
    REFUSED.swap(NOTHING_REFUSED, Ordering::Relaxed)
}

pub(super) static REFUSED: AtomicI64 = AtomicI64::new(NOTHING_REFUSED);

pub fn is_single_instance(service: &[u8]) -> bool {
    SINGLE_INSTANCE.iter().any(|s| *s == service)
}
