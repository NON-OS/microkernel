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

//! The splash while up: a key toggles the attestation detail, the status
//! line spins, and it ends when `dwell` says the desktop has it, or with
//! the compositor's new size when its canvas changes (`follow`).

use nonos_libc::{mk_ipc_recv_from, mk_uptime_ms, mk_yield};

use crate::follow::{Follow, Size};
use crate::look::Look;
use crate::{display, dwell, keys, proto};

/// Runs the splash on its surface at `base`; None when it is over, or the
/// compositor's new canvas size to draw it again at.
pub(crate) fn interact(comp: u32, base: u64, look: &Look, f: &mut Follow) -> Option<Size> {
    let (w, h, stride) = f.size;
    let spx = stride as usize / 4;
    let mut rx = [0u8; 64];
    let mut sender = 0u32;
    let mut show_detail = false;
    let mut frame: u32 = u32::MAX;
    loop {
        let now = mk_uptime_ms();
        if f.start < 0 {
            f.start = now;
        }
        let el = if f.start >= 0 && now >= f.start { now - f.start } else { 0 };
        if f.desktop_up_at < 0 && proto::lookup(b"desktop_shell").is_some() {
            f.desktop_up_at = now;
        }
        if dwell::over(el, now, f.desktop_up_at, show_detail, f.iters) {
            crate::say::stepped_aside(now, f.desktop_up_at);
            return None;
        }
        f.iters = f.iters.saturating_add(1);
        if f.due(now) {
            if let Some(size) = f.seen(now, display::query(comp, 30)) {
                return Some(size);
            }
        }
        let n = mk_ipc_recv_from(0, rx.as_mut_ptr(), rx.len(), 50, &mut sender);
        if n > 0 {
            keys::key(comp, base, f.size, look, &rx[..n as usize], &mut show_detail);
            continue;
        }
        if !show_detail {
            let tick = (el / 150) as u32;
            if tick != frame {
                frame = tick;
                crate::spin::spin(comp, base, (w, h, spx), look.fit.status, frame);
            }
        }
        mk_yield();
    }
}
