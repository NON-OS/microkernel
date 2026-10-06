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

use core::sync::atomic::{AtomicU32, Ordering};

use spin::{Mutex, MutexGuard};

use crate::browser::manifest::{HEIGHT, WIDTH};

/* The page viewport a cascade resolves vw/vh units and media features
 * against, and the root element's font size that rem resolves against.
 * Length parsing runs deep inside the per-declaration appliers, which carry
 * only the element's font size, so these are published here for the length
 * of one cascade instead of being threaded through every applier.
 *
 * A cascade takes the lock, publishes its viewport and keeps the lock until
 * it returns, so two cascades on different threads (the host proofs run in
 * parallel) never read each other's values. The capsule is single threaded
 * and never contends. Until a cascade publishes, the manifest's window
 * size stands; only the host harness's direct compute() reads it so. */
static CASCADE: Mutex<()> = Mutex::new(());
static VW: AtomicU32 = AtomicU32::new(WIDTH);
static VH: AtomicU32 = AtomicU32::new(HEIGHT);
/* The root font size as f32 bits, from the CSS initial 16px. */
const INITIAL_FS: f32 = 16.0;
static ROOT_FS: AtomicU32 = AtomicU32::new(INITIAL_FS.to_bits());

/// Proof that the caller owns the published values; dropping it ends the
/// cascade's hold.
pub struct Held {
    _guard: MutexGuard<'static, ()>,
}

/// Publish `w` x `h` for the cascade the caller is about to run, with the
/// root font size back at its initial value until the root element sets it.
pub fn enter(w: u32, h: u32) -> Held {
    let guard = CASCADE.lock();
    VW.store(w, Ordering::Relaxed);
    VH.store(h, Ordering::Relaxed);
    set_root_font(INITIAL_FS);
    Held { _guard: guard }
}

/// Viewport width in px for the running cascade.
pub fn width() -> u32 {
    VW.load(Ordering::Relaxed)
}

/// Viewport height in px for the running cascade.
pub fn height() -> u32 {
    VH.load(Ordering::Relaxed)
}

/// Record the root element's computed font size, the size of one rem.
pub fn set_root_font(px: f32) {
    ROOT_FS.store(px.to_bits(), Ordering::Relaxed);
}

/// One rem in px for the running cascade.
pub fn root_font() -> f32 {
    f32::from_bits(ROOT_FS.load(Ordering::Relaxed))
}
