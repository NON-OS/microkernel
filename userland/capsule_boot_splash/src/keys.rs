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
//! A key while the splash is up: D toggles the attestation detail; any
//! other key goes back to the splash.

use nonos_libc::INPUT_KIND_KEY_DOWN;

use crate::follow::Size;
use crate::look::Look;
use crate::{detail, input, paint, scene};

pub(crate) fn key(comp: u32, base: u64, size: Size, look: &Look, msg: &[u8], detail_on: &mut bool) {
    let (w, h, stride) = size;
    let Some((kind, code)) = input::parse_key(msg) else { return };
    if kind != INPUT_KIND_KEY_DOWN {
        return;
    }
    *detail_on = (code == b'D' as u32 || code == b'd' as u32) && !*detail_on;
    if *detail_on {
        detail::detail(base, w, h, stride, &look.att);
    } else {
        paint::splash(base, w, h, stride, look);
    }
    let _ = scene::damage(comp, 20, (0, 0, w, h));
}
