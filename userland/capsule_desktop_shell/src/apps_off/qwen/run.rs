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

/* Opening Qwen's window on the tier chosen, or the default. */

use core::sync::atomic::{AtomicI64, AtomicU8, Ordering};

use alloc::vec::Vec;
use nonos_libc::mk_tool_run;
use nonos_policy_proto::Field;

use super::default_tier::{resolve, Source, WEIGHTS};
use super::{memory, need};
use crate::server::handlers::launcher_request::LaunchOutcome;

pub const SERVICE: &[u8] = b"tool.qwen";
const WINDOW: &[u8] = b"window";

pub(super) static LAST: AtomicI64 = AtomicI64::new(0);
/* The default the last open ran, as an index into WEIGHTS plus one; 0 for the person's choice. */
pub(super) static DEFAULT: AtomicU8 = AtomicU8::new(0);

pub fn open() -> LaunchOutcome {
    let port = nonos_policy_client::lookup();
    let mut chosen = [0u8; 32];
    let n = port
        .and_then(|port| nonos_policy_client::get_str(port, Field::QwenTier, &mut chosen))
        .unwrap_or(0)
        .min(chosen.len());
    let room = match port.and_then(|p| nonos_policy_client::get_bool(p, Field::Persistent)) {
        Some(true) => need::Room::Disk,
        _ => need::Room::Memory,
    };
    let (tier, source) = resolve(&chosen[..n], memory::memory(), room);
    let at = match source {
        Source::Chosen => 0,
        _ => WEIGHTS.iter().position(|w| w.0 == tier).map_or(0, |i| i + 1),
    };
    DEFAULT.store(at as u8, Ordering::Relaxed);
    let tier = tier.as_bytes();
    let mut argv = Vec::with_capacity(WINDOW.len() + 1 + tier.len());
    argv.extend_from_slice(WINDOW);
    argv.push(0);
    argv.extend_from_slice(tier);
    let rc = mk_tool_run(SERVICE, &argv);
    LAST.store(rc, Ordering::Relaxed);
    if rc < 0 {
        LaunchOutcome::Failed
    } else {
        LaunchOutcome::Queued
    }
}
