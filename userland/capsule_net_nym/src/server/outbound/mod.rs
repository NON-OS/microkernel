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
//! The sender. Every packet for the gateway waits here for the sending
//! clock, and when none waits the clock sends cover, so the link carries the
//! same Poisson stream whether or not anyone is using it (step.rs). A send
//! is answered when its frames are queued; if a frame later cannot be
//! written, the link is dropped, everything queued goes with it, and the
//! next OP_SEND on that session returns E_NO_TCP once (`take_failure`).

extern crate alloc;

mod cover;
mod done;
mod queue;
mod step;

use alloc::collections::VecDeque;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicI64, AtomicU64, AtomicU8, Ordering};

use nonos_libc::mk_uptime_ms;
use spin::Mutex;

pub use done::Done;
pub use queue::Frame;
use queue::{Job, Queue};
use step::{step, Step};

use crate::crypto::random::fill_random;
use crate::gateway_client;
use crate::mixnet::cover_clock::{code_of, profile_of, Clocks, Profile, Tick, LOW_RATE};
use crate::protocol::{E_BUSY, E_CRYPTO, E_OK};
use crate::state::TABLE;
use crate::{setup, trace};

static QUEUE: Mutex<Queue<Done>> = Mutex::new(Queue::new());
static CLOCKS: Mutex<Option<Clocks>> = Mutex::new(None);
static PROFILE: AtomicU8 = AtomicU8::new(0);
/// Sessions whose frames were dropped unwritten, told on their next send.
static FAILED: Mutex<Vec<(u32, u32)>> = Mutex::new(Vec::new());

/// Packets out since the last minute line: real, drop cover, loop cover.
static SENT: [AtomicU64; 3] = [AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0)];
static SAID_MS: AtomicI64 = AtomicI64::new(0);
const SAY_EVERY_MS: i64 = 60_000;

/// Once a minute while the link is up, what left: the line the project reads
/// to see the clocks running and cover leaving while nobody uses the link.
fn say_minute() {
    let now = mk_uptime_ms();
    if now - SAID_MS.load(Ordering::Relaxed) < SAY_EVERY_MS {
        return;
    }
    SAID_MS.store(now, Ordering::Relaxed);
    let [real, drop, looped] = [0, 1, 2].map(|i| SENT[i].swap(0, Ordering::Relaxed));
    trace::say_two(b"sender minute: real, cover", real, drop + looped);
    trace::say_two(b"sender minute: drop cover, loop cover", drop, looped);
}

/// Ticks handled in one turn of the serve loop at most, so a loop that was
/// held up catches up without starving the clients it serves.
const TICKS_PER_TURN: usize = 32;

fn profile() -> Profile {
    profile_of(PROFILE.load(Ordering::Relaxed)).unwrap_or(LOW_RATE)
}

/// Choose the clock rates (0 low, 1 Nym's). The clocks start again on it.
pub fn set_profile(code: u8) -> bool {
    if profile_of(code).is_none() {
        return false;
    }
    PROFILE.store(code, Ordering::Relaxed);
    *CLOCKS.lock() = None;
    true
}

pub fn profile_code() -> u8 {
    code_of(profile())
}

/// Queue `frames` for `owner`'s session; E_BUSY when they do not fit.
pub fn enqueue(owner: Option<(u32, u32)>, frames: Vec<Frame>, done: Done) -> u16 {
    let job = Job { owner, frames: frames.into_iter().collect::<VecDeque<_>>(), done };
    match QUEUE.lock().push(job) {
        Ok(()) => E_OK,
        Err(_) => E_BUSY,
    }
}

/// Whether frames of this session were dropped unwritten since it last
/// asked; asking clears it.
pub fn take_failure(pid: u32, id: u32) -> bool {
    let mut failed = FAILED.lock();
    let before = failed.len();
    failed.retain(|&s| s != (pid, id));
    failed.len() != before
}

/// The link went away: what was queued for it goes, its clients are told.
pub fn lost() {
    let dropped = QUEUE.lock().drain();
    mark(dropped.iter().filter_map(|j| j.owner));
}

fn mark(owners: impl Iterator<Item = (u32, u32)>) {
    let mut failed = FAILED.lock();
    for o in owners {
        if !failed.contains(&o) {
            failed.push(o);
        }
    }
}

fn random() -> u64 {
    let mut b = [0u8; 8];
    // A failed draw leaves zeros, which exp_delay_ns reads as its longest
    // gap: the clock slows, it never fires early.
    let _ = fill_random(&mut b);
    u64::from_le_bytes(b)
}

fn now_ns() -> u64 {
    (mk_uptime_ms().max(0) as u64).saturating_mul(1_000_000)
}

/// Milliseconds the serve loop may wait before the next tick, at most `idle`.
pub fn wait_ms(idle: u64) -> u64 {
    match *CLOCKS.lock() {
        Some(c) => (c.next_ns().saturating_sub(now_ns()) / 1_000_000).min(idle),
        None => idle,
    }
}

/// When the next tick falls, in uptime milliseconds; zero before the clocks run.
pub fn next_tick_ms() -> u64 {
    CLOCKS.lock().map_or(0, |c| c.next_ns() / 1_000_000)
}

/// Let out whatever the clocks say is due. Nothing leaves without a link.
pub fn tick() {
    let Some(gateway) = TABLE.lock().gateway() else {
        *CLOCKS.lock() = None;
        return;
    };
    if !crate::server::connected() {
        *CLOCKS.lock() = None;
        return;
    }
    say_minute();
    let tcp_port = setup::tcp_port();
    let profile = profile();
    for _ in 0..TICKS_PER_TURN {
        let now = now_ns();
        let mut clocks = match *CLOCKS.lock() {
            Some(c) => c,
            None => Clocks::start(now, profile, random(), random()),
        };
        let write = |frame: &Frame| -> Result<(), u16> {
            let sealed;
            let bytes = match frame {
                Frame::Raw(raw) => raw.as_slice(),
                Frame::Sphinx(packet) => {
                    sealed = gateway_client::make_encrypted_blob(
                        gateway_client::KIND_FORWARD_SPHINX,
                        packet,
                    )
                    .map_err(|_| E_CRYPTO)?;
                    sealed.as_slice()
                }
            };
            gateway_client::send(tcp_port, gateway, bytes)
        };
        let outcome =
            step(now, profile, &mut clocks, &mut QUEUE.lock(), random(), cover::cover, write);
        *CLOCKS.lock() = Some(clocks);
        match outcome {
            Step::Idle => return,
            Step::Real { done } => {
                SENT[0].fetch_add(1, Ordering::Relaxed);
                if let Some((_, d)) = done {
                    done::finish(d);
                }
            }
            Step::Cover(Tick::Send) => {
                SENT[1].fetch_add(1, Ordering::Relaxed);
            }
            Step::Cover(Tick::Loop) => {
                SENT[2].fetch_add(1, Ordering::Relaxed);
            }
            Step::NoCover(Tick::Loop) => {
                trace::say(b"cover: no loop cover could be built, tick passed empty")
            }
            Step::NoCover(Tick::Send) => {
                trace::say(b"cover: no drop cover could be built, tick passed empty")
            }
            Step::Failed { code, dropped, owner } => {
                gateway_client::trace::fail(b"send", code);
                mark(dropped.iter().filter_map(|j| j.owner).chain(owner));
                *CLOCKS.lock() = None;
                crate::server::gateway_lost();
                return;
            }
        }
    }
}
