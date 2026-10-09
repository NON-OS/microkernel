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
 * Qwen from the dock or the Launchpad: the conversation in a window of its
 * own, on the tier chosen at setup or in Settings, as `qwen window` opens it
 * from the Terminal. With none chosen, the default, by the rule the
 * Terminal and setup share (`default_tier`: the stick tier when it fits,
 * else the largest that fits), and a toast says it is the default. The
 * kernel queues the run for init, so a click that succeeds has no pid to
 * give and the window appears a moment later.
 */
use core::sync::atomic::{AtomicI64, AtomicU8, Ordering};

use alloc::vec::Vec;
use nonos_libc::mk_tool_run;
use nonos_policy_proto::Field;

use crate::server::handlers::launcher_request::LaunchOutcome;

/* Which tier runs when none is named, and the fit rule it weighs by. */
#[path = "../../../capsule_model_fetch/src/default_tier.rs"]
mod default_tier;
#[path = "../../../capsule_model_fetch/src/need.rs"]
mod need;
/* The memory the kernel counts, read as `qwen tiers` reads it. */
#[path = "../../../capsule_model_fetch/src/tiers/memory.rs"]
mod memory;

use default_tier::{resolve, Source, WEIGHTS};

pub const SERVICE: &[u8] = b"tool.qwen";
const WINDOW: &[u8] = b"window";

static LAST: AtomicI64 = AtomicI64::new(0);
/* The default the last open ran, as an index into WEIGHTS plus one; 0 for the person's choice. */
static DEFAULT: AtomicU8 = AtomicU8::new(0);

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

/*
 * Said once a window opened on a default: "Qwen: none chosen, default
 * qwen3-0.6b", short enough for a toast. None for the person's choice.
 */
pub fn default_said() -> Option<Vec<u8>> {
    let at = usize::from(DEFAULT.swap(0, Ordering::Relaxed)).checked_sub(1)?;
    Some([&b"Qwen: none chosen, default "[..], WEIGHTS[at].0.as_bytes()].concat())
}

/* The kernel's answer to the last open, for the log line beside `why`. */
pub fn last() -> i64 {
    LAST.load(Ordering::Relaxed)
}

/* Why the last click opened nothing, as the Terminal's qwen says it. A tier
 * with no model is not a refusal: the window opens and says so itself. */
pub fn why() -> &'static [u8] {
    match LAST.load(Ordering::Relaxed) {
        -2 => b"Qwen is not in this system",
        -22 => b"Qwen: the kernel does not know this tier",
        -16 => b"Qwen: every chat place is taken; close one",
        -28 => b"Qwen: no room to start another program",
        -12 => b"Qwen: not enough memory for this model",
        -11 => b"Qwen: the system is busy; try again",
        -13 => b"Linux and Qwen turned off at setup",
        _ => b"Qwen could not start; qwen in Terminal says why",
    }
}
