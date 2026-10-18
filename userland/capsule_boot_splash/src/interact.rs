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

//! The splash while it is up: a key toggles the attestation detail, the
//! status line spins, and it ends when `dwell` says the desktop has it.

use nonos_libc::{mk_ipc_recv_from, mk_uptime_ms, mk_yield, INPUT_KIND_KEY_DOWN};

use crate::look::Look;
use crate::{detail, dwell, input, paint, proto, scene};

pub(crate) fn interact(comp: u32, base: u64, w: u32, h: u32, stride: u32, look: &Look) {
    let spx = stride as usize / 4;
    let mut rx = [0u8; 64];
    let mut sender = 0u32;
    let mut show_detail = false;
    let mut iters: u32 = 0;
    let mut frame: u32 = u32::MAX;
    let mut start = mk_uptime_ms();
    let mut desktop_up_at: i64 = -1;
    loop {
        let now = mk_uptime_ms();
        if start < 0 {
            start = now;
        }
        let el = if start >= 0 && now >= start { now - start } else { 0 };
        if desktop_up_at < 0 && proto::lookup(b"desktop_shell").is_some() {
            desktop_up_at = now;
        }
        if dwell::over(el, now, desktop_up_at, show_detail, iters) {
            break;
        }
        iters = iters.saturating_add(1);
        let n = mk_ipc_recv_from(0, rx.as_mut_ptr(), rx.len(), 50, &mut sender);
        if n > 0 {
            if let Some((kind, code)) = input::parse_key(&rx[..n as usize]) {
                if kind == INPUT_KIND_KEY_DOWN {
                    show_detail = (code == b'D' as u32 || code == b'd' as u32) && !show_detail;
                    if show_detail {
                        detail::detail(base, w, h, stride, &look.att);
                    } else {
                        paint::splash(base, w, h, stride, look);
                    }
                    let _ = scene::damage(comp, 20, w, h);
                }
            }
            continue;
        }
        if !show_detail {
            let f = (el / 150) as u32;
            if f != frame {
                frame = f;
                let buf =
                    unsafe { core::slice::from_raw_parts_mut(base as *mut u32, spx * h as usize) };
                paint::status(buf, spx, w, h, frame);
                let _ = scene::damage(comp, 21, w, h);
            }
        }
        mk_yield();
    }
}
