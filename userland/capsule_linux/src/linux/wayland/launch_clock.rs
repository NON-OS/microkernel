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

//! How long a Linux window takes to come, from its family's start and from
//! its display connection, and how much of that the bridge spent serving;
//! then what its frames cost the bridge, every 256 frames: time, bytes read
//! from the guest and bytes laid inside the bridge (frame_cost.rs).

use core::sync::atomic::{AtomicU64, Ordering::Relaxed};

use crate::linux::call::now_ms;

const EVERY: u64 = 256;
static CONNECTED: AtomicU64 = AtomicU64::new(u64::MAX);
static SERVED: [AtomicU64; 2] = [AtomicU64::new(0), AtomicU64::new(0)];
static FIRST_SAID: AtomicU64 = AtomicU64::new(0);
/* Frames, milliseconds presenting them, bytes read, those shown whole, and
 * bytes laid. */
static FRAMES: [AtomicU64; 5] = [const { AtomicU64::new(0) }; 5];

/// Milliseconds since the family started.
pub fn now() -> u64 {
    now_ms(1).unwrap_or(0)
}

/// The first client connected to the display.
pub fn connected() {
    let _ = CONNECTED.compare_exchange(u64::MAX, now(), Relaxed, Relaxed);
}

/// The bridge served `bytes` of requests since `t0`; the first frame's note
/// once one has been shown.
pub fn served(t0: u64, bytes: usize) {
    if FIRST_SAID.load(Relaxed) != 0 {
        return;
    }
    let t = now();
    SERVED[0].fetch_add(t.saturating_sub(t0), Relaxed);
    SERVED[1].fetch_add(bytes as u64, Relaxed);
    if FRAMES[0].load(Relaxed) != 0 && FIRST_SAID.swap(1, Relaxed) == 0 {
        let (ms, kib) = (SERVED[0].load(Relaxed), SERVED[1].load(Relaxed) / 1024);
        let after = t.saturating_sub(CONNECTED.load(Relaxed).min(t));
        say(alloc::format!("[WAYLAND] first frame at {t} ms, {after} ms after connect; bridge {ms} ms, {kib} KiB\n"));
    }
}

/// `bytes` of pixels laid inside the bridge for the frame being shown.
pub fn laid(bytes: u64) {
    FRAMES[4].fetch_add(bytes, Relaxed);
}

/// A frame shown since `t0`, with `read` bytes read from the guest.
pub fn shown(t0: u64, read: u64, whole: bool) {
    let n = FRAMES[0].fetch_add(1, Relaxed) + 1;
    FRAMES[1].fetch_add(now().saturating_sub(t0), Relaxed);
    FRAMES[2].fetch_add(read, Relaxed);
    FRAMES[3].fetch_add(u64::from(whole), Relaxed);
    if n.is_multiple_of(EVERY) {
        let [ms, read, whole, laid] = [1, 2, 3, 4].map(|i| FRAMES[i].swap(0, Relaxed));
        let (us, kib, laid) = (ms * 1000 / EVERY, read / 1024 / EVERY, laid / 1024 / EVERY);
        say(alloc::format!(
            "[WAYLAND] {EVERY} frames, {whole} whole: {us} us, {kib} KiB read and {laid} KiB laid a frame\n"
        ));
    }
}

fn say(line: alloc::string::String) {
    crate::linux::say::note(line.as_bytes());
}
