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
//! Output 0's monitor, once, at bring-up: its EDID read when the host
//! offered it, and kept as the wire's panel (its modes that fit, its
//! preferred mode and physical size). A host without EDID, or a blob that
//! does not parse, leaves an empty panel: the host's own size still leads
//! MODES, and the boot log says which it was.

use nonos_gfx_serve::Panel;
use nonos_libc::mk_debug;

use crate::device::cmd::get_edid;
use crate::device::cmd::get_edid::EDID_MAX_LEN;
use crate::device::ControlQueue;
use crate::display::modes::fits;

pub fn panel(q: &ControlQueue, offered: bool) -> Panel {
    let mut blob = [0u8; EDID_MAX_LEN];
    let read = if offered { get_edid(q, 0, &mut blob) } else { Err("not offered") };
    let p = read.map(|n| Panel::read(&blob[..n], fits)).unwrap_or_default();
    let line: &[u8] = match (read, p.len) {
        (Err(_), _) => b"[GPU-EDID] no EDID: the host's size leads",
        (Ok(_), 0) => b"[GPU-EDID] EDID read, no mode that fits",
        (Ok(_), _) => b"[GPU-EDID] EDID read: its modes offered",
    };
    mk_debug(line.as_ptr(), line.len());
    p
}
